//! Native application flows with controlled provider IO. The fixture records calls; SQLite and
//! application code decide the outcome. No credentials or real cloud resources are involved.
use super::{SessionRecord, Store, WorkerRecord};
use crate::adapters::{aws, kubevirt};
use crate::app::{session, worker};
use crate::domain::{manifest, session::SessionState};
use anyhow::{Context, Result, bail};
use aws_sdk_ec2::types::InstanceStateName;
use b10x_substrate_sdk::{ExecState, SdkError};
use mantle_conformance::Reply;
use serde_json::{Value, json};
use std::cell::{Cell, RefCell};
use std::time::Duration;

#[allow(dead_code)]
#[path = "../../../mantle-worker/tests/support/upgrade.rs"]
mod upgrade_fixture;

pub(super) fn operator(command: &str, input: &Value) -> Result<Reply> {
    if command == "mantle.operator.AssessAcceptance" {
        return acceptance(input);
    }
    if command == "mantle.operator.UpgradeWorker" {
        return upgrade_worker(input);
    }
    if command == "mantle.operator.VerifyRelease" || command == "mantle.operator.InstallRelease" {
        return release(command, input);
    }
    if command == "mantle.operator.DiagnoseWorker" {
        return doctor(input);
    }
    use crate::profile::{Profile, Registry, Selection};
    use std::os::unix::fs::{PermissionsExt, symlink};
    let temporary = tempfile::tempdir()?;
    let home = temporary.path();
    let registry = Registry::new(home);
    let text = |key| input[key].as_str().unwrap_or("");
    let flag = |key| input[key].as_bool().unwrap_or(false);
    match command {
        "mantle.operator.ProfileRegistry" => {
            let profile = Profile {
                name: text("name").into(),
                config_path: if flag("relative") {
                    text("config").into()
                } else {
                    home.join(text("config"))
                },
                state_dir: home.join(text("state")),
            };
            let path = home
                .join(".config/mantle/profiles")
                .join(format!("{}.toml", profile.name));
            if flag("existing") {
                registry.add(&profile)?;
            }
            let before = std::fs::read(&path).ok();
            if flag("symlink") {
                std::fs::remove_file(&path)?;
                let target = home.join("sentinel");
                std::fs::write(&target, b"fixture sentinel")?;
                symlink(target, &path)?;
            }
            let mut observed = None;
            let result = match text("action") {
                "add" => registry.add(&profile),
                "show" => registry.show(&profile.name).map(|p| {
                    observed = Some(p);
                }),
                "list" => registry.list().map(|profiles| {
                    observed = profiles.into_iter().next();
                }),
                other => bail!("unknown profile fixture action {other}"),
            };
            let profiles = registry.list().unwrap_or_default();
            let paths_match = observed
                .as_ref()
                .or_else(|| profiles.first())
                .is_some_and(|p| {
                    p.name == profile.name
                        && p.config_path == profile.config_path
                        && p.state_dir == profile.state_dir
                });
            let private = path
                .symlink_metadata()
                .is_ok_and(|meta| meta.is_file() && meta.permissions().mode() & 0o077 == 0);
            Ok(Reply::returned(
                json!({"accepted":result.is_ok(),"count":profiles.len(),"paths_match":paths_match,"private":private,"state_created":profile.state_dir.exists(),"unchanged":before.is_some() && before == std::fs::read(&path).ok()}),
            ))
        }
        "mantle.operator.SelectProfile" => {
            for name in ["one", "two"] {
                registry.add(&Profile {
                    name: name.into(),
                    config_path: home.join(format!("{name}.toml")),
                    state_dir: home.join(name),
                })?;
            }
            let nonempty = |key| (!text(key).is_empty()).then(|| text(key));
            let result = Selection::resolve(
                home,
                nonempty("explicit"),
                nonempty("environment"),
                flag("legacy_config").then(|| home.join("legacy.toml").into_os_string()),
                flag("legacy_state").then(|| home.join("legacy-state").into_os_string()),
            );
            let selected = result
                .as_ref()
                .ok()
                .and_then(|s| s.profile.as_deref())
                .unwrap_or("");
            let paths_match = result.as_ref().is_ok_and(|s| {
                let (config, state) = if let Some(name) = &s.profile {
                    (format!("{name}.toml"), name.clone())
                } else {
                    (
                        if flag("legacy_config") {
                            "legacy.toml"
                        } else {
                            ".config/mantle/config.toml"
                        }
                        .into(),
                        if flag("legacy_state") {
                            "legacy-state"
                        } else {
                            ".local/state/mantle"
                        }
                        .into(),
                    )
                };
                s.config_path == home.join(config) && s.state_dir == home.join(state)
            });
            Ok(Reply::returned(
                json!({"accepted":result.is_ok(),"selected":selected,"paths_match":paths_match,"state_created":home.join("one").exists() || home.join("two").exists() || home.join("legacy-state").exists()}),
            ))
        }
        "mantle.operator.ProfileStateIsolation" => {
            let mut paths = Vec::new();
            let mut stores = Vec::new();
            for name in ["one", "two"] {
                registry.add(&Profile {
                    name: name.into(),
                    config_path: home.join(format!("{name}.toml")),
                    state_dir: home.join(name),
                })?;
                let selection = Selection::resolve(home, Some(name), None, None, None)?;
                selection.prepare_state()?;
                paths.push(selection.state_dir.join("state.db"));
                let store = Store::open(paths.last().unwrap())?;
                let mut row = record(SessionState::Running);
                row.name = name.into();
                store.insert_session(&row)?;
                stores.push(store);
            }
            let names = |store: &Store| -> Result<Vec<String>> {
                Ok(store.live_sessions()?.into_iter().map(|r| r.name).collect())
            };
            Ok(Reply::returned(
                json!({"first_names":names(&stores[0])?,"second_names":names(&stores[1])?,"paths_distinct":paths[0] != paths[1]}),
            ))
        }
        other => bail!("unsupported operator command {other}"),
    }
}

fn upgrade_worker(input: &Value) -> Result<Reply> {
    use mantle_worker::upgrade;
    let f = upgrade_fixture::Fixture::new();
    let before = f.snapshot();
    let case = input["case"].as_str().context("upgrade case")?;
    match case {
        "active-refusal" => *f.host.fault.borrow_mut() = "active".into(),
        "unknown-refusal" => *f.host.fault.borrow_mut() = "unknown".into(),
        "incompatible" => *f.host.fault.borrow_mut() = "incompatible".into(),
        "tampered" => {
            let manifest: Value = serde_json::from_slice(&std::fs::read(&f.manifest)?)?;
            let archive = manifest["artifacts"][0]["name"]
                .as_str()
                .context("archive")?;
            std::fs::write(f.manifest.parent().unwrap().join(archive), b"tampered")?;
        }
        "plan" | "current" | "apply" | "rollback" | "postcheck" => {}
        other => bail!("unsupported upgrade fixture {other}"),
    }
    struct Failure<'a>(&'a str);
    impl upgrade::Hooks for Failure<'_> {
        fn boundary(&self, name: &str) -> Result<()> {
            anyhow::ensure!(
                name != "postcheck" && !(self.0 == "postcheck" && name == "before-rollback"),
                "controlled postcondition failure"
            );
            Ok(())
        }
    }
    let result = match case {
        "current" => {
            upgrade::apply(&f.host, &f.root, &f.manifest, &())?;
            upgrade::check(&f.host, &f.root, &f.manifest)
        }
        "rollback" | "postcheck" => upgrade::apply(&f.host, &f.root, &f.manifest, &Failure(case)),
        "apply" | "tampered" => upgrade::apply(&f.host, &f.root, &f.manifest, &()),
        _ => upgrade::check(&f.host, &f.root, &f.manifest),
    };
    let outcome = match result {
        Ok(report) => report.outcome,
        Err(_) if case == "tampered" => "refused".into(),
        Err(error) => return Err(error),
    };
    let previous_preserved = f.snapshot() == before
        || std::fs::read_dir(&f.root)?
            .filter_map(Result::ok)
            .any(|entry| {
                entry.file_name().to_string_lossy().starts_with(".upgrade-")
                    && before.iter().all(|(name, bytes)| {
                        let path = entry.path().join("bin").join(name);
                        if name == "codex" {
                            std::fs::read_link(path)
                                .is_ok_and(|p| p.as_os_str().as_encoded_bytes() == bytes)
                        } else {
                            std::fs::read(path).is_ok_and(|actual| actual == *bytes)
                        }
                    })
            });
    let services_unchanged = f.host.calls.borrow().iter().all(|c| {
        !c.contains(" stop")
            && !c.contains(" start")
            && !c.contains(" mask")
            && !c.contains(" restart")
    });
    Ok(Reply::returned(
        json!({"outcome":outcome,"previous_preserved":previous_preserved,"services_unchanged":services_unchanged}),
    ))
}

fn doctor(input: &Value) -> Result<Reply> {
    use crate::app::doctor::{self, Probes};
    use crate::config::RuntimeContext;
    struct Probe {
        failure: String,
        calls: RefCell<Vec<String>>,
    }
    impl Probe {
        fn call(&self, stage: &str) -> Result<()> {
            self.calls.borrow_mut().push(stage.into());
            anyhow::ensure!(
                self.failure != stage && !(stage == "provider" && self.failure == "timeout"),
                "controlled probe refusal"
            );
            Ok(())
        }
    }
    impl Probes for Probe {
        fn provider(&self, _: &RuntimeContext, _: &WorkerRecord, _: Duration) -> Result<()> {
            self.call("provider")
        }
        fn ssh(&self, _: &RuntimeContext, _: &WorkerRecord, _: Duration) -> Result<()> {
            self.call("ssh")
        }
        fn service(&self, _: &RuntimeContext, _: &WorkerRecord, _: Duration) -> Result<()> {
            self.call("service")
        }
        async fn compatibility(
            &self,
            _: &RuntimeContext,
            _: &WorkerRecord,
            _: Duration,
        ) -> Result<b10x_substrate_sdk::Machine> {
            self.call("compatibility")?;
            let scenario: Value = serde_json::from_str(include_str!(
                "../../../../spec/scenarios/cli/orchestration-observe-all-required-facts.yaml"
            ))?;
            let facts: Value = if self.failure == "facts" {
                json!({"operation.ledger-subject-max-rows":1000,"operation.ledger-subject-max-bytes":1048576,"operation.ledger-global-max-rows":10000,"operation.ledger-global-max-bytes":10485760})
            } else {
                serde_json::from_str(
                    scenario["timeline"][0]["input"]["facts_json"]
                        .as_str()
                        .unwrap(),
                )?
            };
            Ok(serde_json::from_value(
                json!({"capability_snapshot":"fixture","driver_version":if self.failure=="version" {"0.0.0"} else {worker::SUBSTRATE_VERSION},"configuration_generation":1,"probed_at":"2026-10-03T00:00:00Z","valid_until":null,"facts":facts,"guarded_workspace_io":true,"exec_argv_only":true,"exec_no_egress":true,"exec_cgroup_limits":true,"exec_cgroup_kill":true,"events_pull":true,"events_stream":true}),
            )?)
        }
    }
    let temporary = tempfile::tempdir()?;
    let failure = input["failure"].as_str().context("failure")?;
    let config = temporary.path().join("config.toml");
    std::fs::write(
        &config,
        if failure == "configuration" {
            "invalid"
        } else {
            "provider='kubevirt'\nubuntu_serial='20260926'\n[kubevirt]\ncontext='fixture'\nnamespace='fixture'\ncpu=2\nmemory_gib=4\nroot_disk_gib=8\ndata_disk_gib=8\n"
        },
    )?;
    let state = temporary.path().join("state");
    let store = if failure == "state" {
        None
    } else {
        crate::config::ensure_private_dir(&state)?;
        let store = Store::open(&state.join("state.db"))?;
        store.put_worker(&WorkerRecord {
            name: worker::WORKER.into(),
            instance: "fixture".into(),
            region: if failure == "placement" {
                "wrong".into()
            } else {
                "kubevirt/fixture/fixture".into()
            },
            data_volume: None,
        })?;
        Some(store)
    };
    let before = store.as_ref().map(|s| s.connection.total_changes());
    let schema = store
        .as_ref()
        .map(|s| {
            s.connection
                .query_row("SELECT group_concat(sql) FROM sqlite_master", [], |row| {
                    row.get::<_, String>(0)
                })
        })
        .transpose()?;
    let probes = Probe {
        failure: failure.into(),
        calls: RefCell::new(vec![]),
    };
    let selection = crate::profile::Selection {
        profile: None,
        config_path: config,
        state_dir: state.clone(),
    };
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    let report = runtime.block_on(doctor::diagnose(
        Ok(selection),
        &probes,
        Duration::from_secs(1),
    ));
    let unchanged = if let Some(store) = store {
        Some(store.connection.total_changes()) == before
            && Some(store.connection.query_row(
                "SELECT group_concat(sql) FROM sqlite_master",
                [],
                |row| row.get::<_, String>(0),
            )?) == schema
            && store.worker(worker::WORKER)?.is_some()
    } else {
        !state.exists()
    };
    Ok(Reply::returned(
        json!({"healthy":report.healthy,"failed_stage":report.checks.iter().find(|c|c.status=="failed").map_or("",|c|c.stage),"called":probes.calls.into_inner(),"state_unchanged":unchanged}),
    ))
}

struct Plane {
    input: Value,
    calls: RefCell<Vec<String>>,
    reads: Cell<usize>,
}

#[test]
fn stop_retains_workspace_and_reserves_name() -> Result<()> {
    let store = Store::in_memory()?;
    let mut record = record(SessionState::Running);
    record.workspace = Some("workspace-1".into());
    record.agent_exec = Some("exec-1".into());
    store.insert_session(&record)?;
    let plane = Plane::new(&json!({"exec":"terminal"}));
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(session::stop_recorded(&plane, &store, &record, "fixture"))?;
    assert!(
        !plane.calls.borrow().iter().any(|call| call == "destroy"),
        "stop must preserve workspace contents"
    );
    assert_eq!(
        store
            .live_session("fixture")?
            .context("retained name was released")?
            .state
            .to_string(),
        "RETAINED"
    );
    Ok(())
}
impl Plane {
    fn new(input: &Value) -> Self {
        Self {
            input: input.clone(),
            calls: RefCell::default(),
            reads: Cell::new(0),
        }
    }
    fn text(&self, key: &str) -> &str {
        self.input[key].as_str().unwrap_or("")
    }
    fn call(&self, name: &str) -> Result<()> {
        self.calls.borrow_mut().push(name.into());
        if self.text("fail") == name {
            bail!("fixture IO refusal at {name}");
        }
        Ok(())
    }
    fn instance(&self, state: InstanceStateName) -> aws::Instance {
        aws::Instance {
            id: "instance-1".into(),
            state,
            instance_type: "fixture".into(),
            data_volume: Some("volume-1".into()),
            availability_zone: None,
        }
    }
    fn vm(&self, status: &str) -> kubevirt::Vm {
        kubevirt::Vm {
            uid: "vm-1".into(),
            run_strategy: self.text("initial").into(),
            printable_status: status.into(),
        }
    }
}
impl session::InitPort for &Plane {
    async fn run(&self, argv: &[&str], network: bool) -> Result<String> {
        self.call(&format!(
            "{}:{}",
            if network { "network" } else { "isolated" },
            argv.join(" ")
        ))?;
        let step = if argv.contains(&"clone") {
            "clone"
        } else if argv.contains(&"checkout") {
            "checkout"
        } else if argv.contains(&"rev-parse") {
            "rev-parse"
        } else {
            "mkdir"
        };
        if self.text("fail") == step {
            bail!("fixture IO refusal at {step}");
        }
        Ok(if step == "rev-parse" {
            self.text("commit").into()
        } else {
            String::new()
        })
    }
}
impl session::StartPort for &Plane {
    type Workspace = Self;
    fn worker_binding(&self) -> WorkerRecord {
        WorkerRecord {
            name: "default".into(),
            instance: "fixture".into(),
            region: "fixture".into(),
            data_volume: None,
        }
    }
    async fn create(&self, _: &str, _: &manifest::Resolved) -> Result<Self> {
        self.call("create")?;
        Ok(*self)
    }
    fn workspace_id<'a>(&self, _: &'a Self) -> &'a str {
        "workspace-1"
    }
    async fn start_agent(&self, _: &Self, _: &session::RunRequest, _: &str) -> Result<String> {
        self.call("agent")?;
        Ok("exec-1".into())
    }
    async fn ready(&self, _: &Self, _: &str) -> Result<()> {
        Ok(())
    }
    async fn attach(&self, _: &Self, _: &str, _: &crate::domain::session::AgentKind) -> Result<()> {
        self.call("attach")
    }
}
// Refusal fields mirror the pinned producer's ErrorDetail and app/responses.rs. The SDK converts
// that producer record to Refusal; unknown operation is also a distinct SDK transport outcome.
fn refusal(code: &str) -> SdkError {
    SdkError::Refusal(
        serde_json::from_value(json!({"class":if code=="resource.not-found" {"Refused"} else {"Failed"}, "code":code,
        "message":match code {"resource.not-found"=>"Resource was not found.","operation.outcome-unknown"=>"Operation was accepted but its terminal outcome is unknown.",_=>"fixture refusal"},
        "retriable":code=="operation.outcome-unknown", "address":if code=="operation.outcome-unknown" {"operation"} else {"resource"},
        "operation_id":if code=="operation.outcome-unknown" {Some("operation-1")} else {None}}))
        .unwrap(),
    )
}
impl session::StopPort for Plane {
    type Exec = ExecState;
    type Workspace = ();
    async fn get_exec(&self, _: &str) -> Result<ExecState, SdkError> {
        self.call("get-exec")
            .map_err(|e| SdkError::Transport(e.to_string()))?;
        match self.text("exec") {
            "missing" => Err(refusal("resource.not-found")),
            "error" => Err(refusal("fixture.error")),
            "terminal" => Ok(ExecState::Exited),
            _ => Ok(ExecState::Running),
        }
    }
    fn terminal(&self, exec: &ExecState) -> bool {
        exec.terminal()
    }
    fn binding<'a>(&self, _: &'a ExecState) -> (&'a str, &'a str) {
        ("exec-1", "workspace-1")
    }
    async fn signal(&self, _: &mut ExecState) -> Result<()> {
        self.call("signal")
    }
    async fn wait(&self, exec: &mut ExecState) -> Result<()> {
        self.call("wait")?;
        *exec = ExecState::Exited;
        Ok(())
    }
    async fn retire(&self, _: ExecState, _: &str) -> Result<bool> {
        self.call("retire")?;
        Ok(true)
    }
    async fn get_workspace(&self, _: &str) -> Result<(), SdkError> {
        self.call("get-workspace")
            .map_err(|e| SdkError::Transport(e.to_string()))?;
        let n = self.reads.get();
        self.reads.set(n + 1);
        let state = if n == 0 {
            self.text("workspace")
        } else {
            self.input["readback"]
                .get(n - 1)
                .and_then(Value::as_str)
                .unwrap_or("present")
        };
        match state {
            "missing" => Err(refusal("resource.not-found")),
            "error" => Err(refusal("fixture.error")),
            _ => Ok(()),
        }
    }
    async fn destroy(&self, _: ()) -> Result<(), SdkError> {
        self.call("destroy")
            .map_err(|e| SdkError::Transport(e.to_string()))?;
        match self.text("destroy") {
            "unknown" => Err(refusal("operation.outcome-unknown")),
            "transport-unknown" => Err(SdkError::UnknownOperation {
                operation_id: "operation-1".into(),
            }),
            "refused" => Err(refusal("fixture.error")),
            _ => Ok(()),
        }
    }
}
impl worker::AwsPort for Plane {
    async fn find_instance(&self, _: &str) -> Result<Option<aws::Instance>> {
        self.call("find")?;
        Ok((self.text("initial") != "missing")
            .then(|| self.instance(InstanceStateName::from(self.text("initial")))))
    }
    async fn ensure_instance_profile(&self) -> Result<bool> {
        self.call("profile")?;
        Ok(self.input["new_profile"].as_bool().unwrap_or(false))
    }
    async fn ensure_security_group(&self) -> Result<String> {
        self.call("group")?;
        Ok("group-1".into())
    }
    async fn ubuntu_ami(&self, _: &str) -> Result<String> {
        self.call("image")?;
        Ok("image-1".into())
    }
    async fn launch(&self, _: &str, _: &str, _: &str, _: &str) -> Result<String> {
        self.call("launch")?;
        Ok("instance-1".into())
    }
    async fn start(&self, _: &str) -> Result<()> {
        self.call("start")
    }
    async fn stop(&self, _: &str) -> Result<()> {
        self.call("stop")
    }
    async fn wait_for_state(
        &self,
        _: &str,
        state: InstanceStateName,
        timeout: Duration,
    ) -> Result<aws::Instance> {
        self.call(&format!("wait-{}-{}", state.as_str(), timeout.as_secs()))?;
        Ok(self.instance(state))
    }
    async fn ensure_idle_stop_alarm(&self, _: &str) -> Result<()> {
        self.call("alarm")
    }
    async fn propagate(&self) {
        self.calls.borrow_mut().push("propagate".into());
    }
}
impl worker::KubevirtPort for Plane {
    fn find(&self, _: &str) -> Result<Option<kubevirt::Vm>> {
        self.call("find")?;
        Ok((self.text("initial") != "missing").then(|| self.vm("Stopped")))
    }
    fn launch(&self, _: &str, _: &str) -> Result<()> {
        self.call("launch")
    }
    fn set_run_strategy(&self, _: &str, strategy: &str) -> Result<()> {
        self.call(&format!("strategy-{strategy}"))
    }
    fn image_url(&self) -> String {
        "https://example.com/fixture.img".into()
    }
    async fn wait_for_status(
        &self,
        _: &str,
        status: &str,
        timeout: Duration,
    ) -> Result<kubevirt::Vm> {
        self.call(&format!("wait-{status}-{}", timeout.as_secs()))?;
        Ok(self.vm(status))
    }
}
fn record(state: SessionState) -> SessionRecord {
    SessionRecord {
        generation: 0,
        id: "session-1".into(),
        name: "fixture".into(),
        worker: "default".into(),
        state,
        manifest_digest: "fixture".into(),
        workspace: None,
        agent_exec: None,
        requested_json: "{}".into(),
        created_at: "2026-10-02T00:00:00Z".into(),
        failure: None,
        agent_kind: crate::domain::session::AgentKind::V0,
        authentication: crate::domain::session::AuthenticationMethod::V1,
    }
}

struct LifecycleExec {
    id: String,
    workspace: String,
    state: ExecState,
}
struct LifecyclePlane {
    root: std::path::PathBuf,
    exec: RefCell<Option<(String, ExecState)>>,
    operation: RefCell<Option<b10x_substrate_sdk::Operation>>,
    starts: Cell<usize>,
    destroys: Cell<usize>,
    failure: Cell<&'static str>,
    requests: RefCell<Vec<Value>>,
}
impl session::StopPort for LifecyclePlane {
    type Exec = LifecycleExec;
    type Workspace = std::path::PathBuf;
    async fn get_exec(&self, id: &str) -> Result<Self::Exec, SdkError> {
        match self.exec.borrow().as_ref() {
            Some((actual, state)) if actual == id => Ok(LifecycleExec {
                id: id.into(),
                workspace: "workspace-1".into(),
                state: *state,
            }),
            _ => Err(refusal("resource.not-found")),
        }
    }
    fn terminal(&self, exec: &Self::Exec) -> bool {
        exec.state.terminal()
    }
    fn binding<'a>(&self, exec: &'a Self::Exec) -> (&'a str, &'a str) {
        (&exec.id, &exec.workspace)
    }
    async fn signal(&self, _: &mut Self::Exec) -> Result<()> {
        anyhow::ensure!(self.failure.get() != "signal", "fixture interrupted signal");
        Ok(())
    }
    async fn wait(&self, exec: &mut Self::Exec) -> Result<()> {
        anyhow::ensure!(
            self.failure.get() != "wait",
            "fixture unresolved termination"
        );
        exec.state = ExecState::Exited;
        *self.exec.borrow_mut() = Some((exec.id.clone(), ExecState::Exited));
        Ok(())
    }
    async fn retire(&self, _: Self::Exec, _: &str) -> Result<bool> {
        self.exec.borrow_mut().take();
        anyhow::ensure!(
            self.failure.get() != "retire",
            "fixture crash after retirement"
        );
        Ok(self.failure.get() != "not-absent")
    }
    async fn get_workspace(&self, id: &str) -> Result<Self::Workspace, SdkError> {
        if id == "workspace-1" && self.root.exists() {
            Ok(self.root.clone())
        } else {
            Err(refusal("resource.not-found"))
        }
    }
    async fn destroy(&self, path: Self::Workspace) -> Result<(), SdkError> {
        self.destroys.set(self.destroys.get() + 1);
        std::fs::remove_dir_all(path).map_err(|e| SdkError::Transport(e.to_string()))?;
        if self.failure.get() == "destroy-unknown" {
            Err(refusal("operation.outcome-unknown"))
        } else {
            Ok(())
        }
    }
}
impl crate::app::lifecycle::AdmissionPort for LifecyclePlane {
    async fn operation(&self, id: &str) -> Result<b10x_substrate_sdk::Operation> {
        let operation = self
            .operation
            .borrow()
            .clone()
            .context("fixture missing operation after reconnect")?;
        anyhow::ensure!(operation.id == id, "fixture operation mismatch");
        Ok(operation)
    }
    async fn admit(
        &self,
        _: &Self::Workspace,
        request: &session::RunRequest,
        operation: &str,
    ) -> Result<String> {
        self.starts.set(self.starts.get() + 1);
        self.requests
            .borrow_mut()
            .push(serde_json::to_value(request)?);
        let id = format!("exec-restart-{}", self.starts.get());
        *self.exec.borrow_mut() = Some((id.clone(), ExecState::Running));
        *self.operation.borrow_mut() = Some(serde_json::from_value(
            json!({"id":operation,"kind":"exec.start","state":"Terminal","resource":id,"result":{"state":"running"},"refusal":null}),
        )?);
        anyhow::ensure!(
            self.failure.get() != "admit-unknown",
            "fixture accepted before response was lost"
        );
        Ok(id)
    }
    async fn ready(&self, _: &mut Self::Exec) -> Result<()> {
        anyhow::ensure!(
            self.failure.get() != "ready",
            "fixture interrupted readiness"
        );
        Ok(())
    }
    fn running(&self, exec: &Self::Exec) -> bool {
        exec.state == ExecState::Running
    }
}

/// Each named native case drives real state transitions and remote-boundary calls. Files under
/// the fixture workspace are observable user bytes, not precomputed expected response fields.
pub(super) fn retained_case(case: &str) -> Result<()> {
    use crate::app::lifecycle as life;
    let temporary = tempfile::tempdir()?;
    let database = temporary.path().join("state.db");
    let mut store = Store::open(&database)?;
    let root = temporary.path().join("workspace");
    std::fs::create_dir_all(root.join(".mantle/home/.codex"))?;
    std::fs::write(root.join("marker"), b"user changes\0\xff")?;
    std::fs::write(
        root.join(".mantle/home/.codex/auth.json"),
        b"fixture login bytes, never real credentials",
    )?;
    let other = temporary.path().join("other-workspace");
    std::fs::create_dir(&other)?;
    std::fs::write(other.join("marker"), b"other session")?;
    let plane = LifecyclePlane {
        root: root.clone(),
        exec: RefCell::new(Some(("exec-1".into(), ExecState::Running))),
        operation: RefCell::new(None),
        starts: Cell::new(0),
        destroys: Cell::new(0),
        failure: Cell::new(""),
        requests: RefCell::new(vec![]),
    };
    let mut r = record(SessionState::Materializing);
    r.workspace = Some("workspace-1".into());
    store.insert_session(&r)?;
    let context = life::LaunchContext {
        version: 1,
        worker: WorkerRecord {
            name: "default".into(),
            instance: "fixture".into(),
            region: "fixture".into(),
            data_volume: None,
        },
        request: session::agent_request(&manifest::parse(include_str!(
            "../../../../examples/codex.yaml"
        ))?),
        cpu_time_secs: b10x_substrate_sdk::MAX_EXEC_DURATION.as_secs(),
        output_bytes: b10x_substrate_sdk::MAX_IO_BYTES,
    };
    let initial = store.initialize_session(&r.id, &serde_json::to_string(&context)?)?;
    if case != "retain-initial-materialization" {
        store.lifecycle_write(
            &r.id,
            &initial,
            SessionState::Running,
            None,
            Some("exec-1"),
            true,
        )?;
    }
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    runtime.block_on(async {
        tokio::time::pause();
        let current=|s:&Store|s.session_by_id("session-1")?.context("session vanished");
        match case {
            "retain-initial-materialization"=>{
                let r=current(&store)?;
                anyhow::ensure!(life::stop(&plane,&store,&r,false).await.is_err(),"stop passed active initialization");
                anyhow::ensure!(life::stop(&plane,&store,&r,true).await.is_err(),"destroy passed active initialization");
                store.assert_attempt(&r.id,&initial)?;
                drop(store);store=Store::open(&database)?;
                anyhow::ensure!(life::stop(&plane,&store,&current(&store)?,false).await.is_err(),"reopen fabricated idle initialization");
            }
            "retain-expected-session-id"=>{
                anyhow::ensure!(store.selected_session("fixture",Some("prior-session")).is_err(),"stale id accepted");
                anyhow::ensure!(plane.starts.get()==0&&plane.destroys.get()==0&&current(&store)?.agent_exec.as_deref()==Some("exec-1"),"stale guard mutated session");
            }
            "retain-migration-reopen" | "retain-legacy-stopped"=>{
                store.connection.execute_batch("DELETE FROM session_lifecycle; PRAGMA user_version=0; UPDATE sessions SET state='STOPPING'; INSERT INTO sources VALUES('session-1','source','https://example.invalid/repo','main','fixed','repo');")?;
                drop(store);store=Store::open(&database)?;
                anyhow::ensure!(current(&store)?.state==SessionState::Destroying && store.attempt("session-1")?.unwrap().intent=="destroy","legacy destructive intent lost");
                life::stop(&plane,&store,&current(&store)?,false).await?;
                drop(store);store=Store::open(&database)?;
                anyhow::ensure!(current(&store)?.state==SessionState::Stopped && store.sources("session-1")?.len()==1 && store.live_session("fixture")?.is_none(),"legacy destroyed row or source changed");
                anyhow::ensure!(life::restart(&plane,&store,&current(&store)?).await.is_err(),"legacy row fabricated restart context");
                let mut next=record(SessionState::Materializing);next.id="next-session".into();store.insert_session(&next)?;
            }
            "retain-interrupted-stop" | "retain-retirement-crash"=>{
                plane.failure.set(if case=="retain-interrupted-stop"{"wait"}else{"retire"});
                anyhow::ensure!(life::stop(&plane,&store,&current(&store)?,false).await.is_err(),"interruption reported success");
                anyhow::ensure!(current(&store)?.state==SessionState::Retaining,"intent not durable");
                drop(store);store=Store::open(&database)?;plane.failure.set("");
                life::stop(&plane,&store,&current(&store)?,false).await?;
                anyhow::ensure!(current(&store)?.state==SessionState::Retained,"retry did not retain");
            }
            "retain-missing-terminal-proof" | "retain-retirement-not-absent"=>{
                if case=="retain-missing-terminal-proof"{plane.exec.borrow_mut().take();}else{plane.failure.set("not-absent");}
                anyhow::ensure!(life::stop(&plane,&store,&current(&store)?,false).await.is_err(),"unproved retirement reported retained");
                anyhow::ensure!(current(&store)?.state==SessionState::Retaining,"unproved stop lost its intent");
            }
            _=>{
                life::stop(&plane,&store,&current(&store)?,false).await?;
                anyhow::ensure!(current(&store)?.state==SessionState::Retained,"stop did not retain");
                anyhow::ensure!(store.insert_session(&record(SessionState::Materializing)).is_err(),"retained name became reusable");
                match case {
                    "retain-stop" | "retain-files-auth"=>{},
                    "retain-repeat-stop"=>life::stop(&plane,&store,&current(&store)?,false).await?,
                    "retain-attach-refusal"=>anyhow::ensure!(life::attachable(&current(&store)?).is_err(),"attach restarted retained session"),
                    "destroy-repeat" | "destroy-readback" | "retain-other-session-survives" | "destroy-explicit-confirmation"=>{
                        if case=="destroy-explicit-confirmation" {
                            use clap::Parser;
                            anyhow::ensure!(crate::Cli::try_parse_from(["mantle","destroy","fixture"]).is_err(),"destroy accepted without confirmation");
                            anyhow::ensure!(crate::Cli::try_parse_from(["mantle","destroy","fixture","--yes"]).is_ok(),"confirmed destroy refused");
                        }
                        if case=="destroy-readback"{plane.failure.set("destroy-unknown");}
                        life::stop(&plane,&store,&current(&store)?,true).await?;
                        life::stop(&plane,&store,&current(&store)?,true).await?;
                        anyhow::ensure!(!root.exists()&&other.join("marker").exists()&&plane.destroys.get()==1,"destruction escaped selected workspace or repeated");
                    }
                    "retain-restart"=>{
                        life::restart(&plane,&store,&current(&store)?).await?;
                        anyhow::ensure!(current(&store)?.agent_exec.as_deref()==Some("exec-restart-1") && plane.starts.get()==1,"restart did not admit fresh exec");
                        anyhow::ensure!(plane.requests.borrow()[0]==serde_json::to_value(&context.request)?,"restart changed launch request");
                    }
                    "retain-restart-failure"=>{
                        std::fs::remove_dir_all(&root)?;
                        anyhow::ensure!(life::restart(&plane,&store,&current(&store)?).await.is_err() && plane.starts.get()==0 && !root.exists(),"missing workspace was recreated");
                    }
                    "retain-unsupported-policy"=>{
                        let mut invalid=serde_json::to_value(&context)?;invalid["version"]=json!(999);
                        store.connection.execute("UPDATE session_lifecycle SET launch_context=?1",[invalid.to_string()])?;
                        anyhow::ensure!(life::restart(&plane,&store,&current(&store)?).await.is_err()&&plane.starts.get()==0,"unsupported launch contract admitted");
                    }
                    "retain-concurrent-restart"=>{
                        let barrier=std::sync::Arc::new(std::sync::Barrier::new(2));
                        let claims=std::thread::scope(|scope| {
                            let handles:Vec<_>=(0..2).map(|_|{
                                let path=&database;let barrier=barrier.clone();
                                scope.spawn(move||->Result<(super::lifecycle::Attempt,bool)>{
                                    let store=Store::open(path)?;
                                    let record=store.session_by_id("session-1")?.context("missing")?;
                                    barrier.wait();store.claim(&record,"admit")
                                })
                            }).collect();
                            handles.into_iter().map(|h|h.join().unwrap()).collect::<Vec<_>>()
                        });
                        let claims:Vec<_>=claims.into_iter().filter_map(Result::ok).filter(|(_,fresh)|*fresh).collect();
                        anyhow::ensure!(claims.len()==1,"concurrent callers won more than one admission");
                        life::admit_recorded(&plane,&store,&current(&store)?,&claims[0].0,&context.request,true).await?;
                        anyhow::ensure!(plane.starts.get()==1,"concurrent admission duplicated");
                    }
                    "retain-restart-before-dispatch" | "retain-unknown-no-duplicate" | "retain-stop-versus-restart" | "retain-destroy-versus-restart" | "retain-stale-completion"=>{
                        let stale=current(&store)?;
                        let (attempt,fresh)=store.claim(&stale,"admit")?;anyhow::ensure!(fresh,"first claim lost");
                        let second=Store::open(&database)?;
                        anyhow::ensure!(second.claim(&stale,"admit").is_err(),"stale second caller claimed start");
                        anyhow::ensure!(life::stop(&plane,&second,&stale,case=="retain-destroy-versus-restart").await.is_err(),"stale stop or destroy acted on admission");
                        anyhow::ensure!(life::restart(&plane,&second,&current(&second)?).await.is_err() && plane.starts.get()==0,"missing operation caused resubmission");
                        if case=="retain-stale-completion" {
                            anyhow::ensure!(second.lifecycle_write("session-1",&initial,SessionState::Stopped,None,None,true).is_err(),"stale completion changed newer attempt");
                        }
                        store.assert_attempt("session-1",&attempt)?;
                    }
                    "retain-restart-accepted-before-persist" | "retain-restart-readiness"=>{
                        plane.failure.set(if case=="retain-restart-readiness"{"ready"}else{"admit-unknown"});
                        anyhow::ensure!(life::restart(&plane,&store,&current(&store)?).await.is_err(),"interruption reported success");
                        if case=="retain-restart-readiness" {anyhow::ensure!(current(&store)?.agent_exec.as_deref()==Some("exec-restart-1"),"exec not persisted before readiness");}
                        drop(store);store=Store::open(&database)?;plane.failure.set("");
                        life::restart(&plane,&store,&current(&store)?).await?;
                        anyhow::ensure!(plane.starts.get()==1 && current(&store)?.state==SessionState::Running,"recovery admitted duplicate");
                    }
                    "retain-failed-admission-cleanup"=>{
                        plane.failure.set("ready");
                        anyhow::ensure!(life::restart(&plane,&store,&current(&store)?).await.is_err(),"failed readiness reported success");
                        let id=current(&store)?.agent_exec.context("admission exec missing")?;
                        *plane.exec.borrow_mut()=Some((id,ExecState::Exited));plane.failure.set("");
                        life::cleanup(&plane,&store,&current(&store)?,false).await?;
                        anyhow::ensure!(current(&store)?.state==SessionState::Retained&&plane.starts.get()==1,"confirmed terminal admission remained uncleansable");
                    }
                    "retain-binding-refusal"=>{
                        plane.failure.set("admit-unknown");
                        anyhow::ensure!(life::restart(&plane,&store,&current(&store)?).await.is_err(),"fixture interruption lost");
                        plane.operation.borrow_mut().as_mut().unwrap().kind="workspace.create".into();
                        anyhow::ensure!(life::restart(&plane,&store,&current(&store)?).await.is_err(),"operation kind mismatch accepted");
                        anyhow::ensure!(life::cleanup(&plane,&store,&current(&store)?,true).await.is_err(),"cleanup bypassed operation binding");
                        anyhow::ensure!(plane.starts.get()==1 && plane.destroys.get()==0,"binding refusal mutated resources");
                    }
                    "retain-fenced-completion-same-state"=>{
                        let stale=current(&store)?;
                        life::restart(&plane,&store,&current(&store)?).await?;
                        life::stop(&plane,&store,&current(&store)?,false).await?;
                        anyhow::ensure!(current(&store)?.state==stale.state && current(&store)?.agent_exec==stale.agent_exec,"fixture did not return to same visible state");
                        anyhow::ensure!(life::cleanup(&plane,&store,&stale,true).await.is_err(),"stale generation destroyed same-state workspace");
                    }
                    _=>bail!("unknown retained lifecycle scenario {case}"),
                }
            }
        }
        if root.exists(){
            anyhow::ensure!(std::fs::read(root.join("marker"))?==b"user changes\0\xff" && std::fs::read(root.join(".mantle/home/.codex/auth.json"))?==b"fixture login bytes, never real credentials","private home or user files changed");
        }
        anyhow::ensure!(std::fs::read(other.join("marker"))?==b"other session","unselected session changed");
        Ok(())
    })
}

#[test]
fn retained_lifecycle_reconciles_durable_intents_and_preserves_files() -> Result<()> {
    for case in [
        "retain-stop",
        "retain-repeat-stop",
        "retain-interrupted-stop",
        "retain-files-auth",
        "retain-restart",
        "retain-restart-failure",
        "retain-attach-refusal",
        "destroy-explicit-confirmation",
        "destroy-readback",
        "destroy-repeat",
        "retain-legacy-stopped",
        "retain-other-session-survives",
        "retain-migration-reopen",
        "retain-restart-before-dispatch",
        "retain-restart-accepted-before-persist",
        "retain-restart-readiness",
        "retain-unknown-no-duplicate",
        "retain-concurrent-restart",
        "retain-stop-versus-restart",
        "retain-destroy-versus-restart",
        "retain-retirement-crash",
        "retain-initial-materialization",
        "retain-stale-completion",
        "retain-unsupported-policy",
        "retain-expected-session-id",
        "retain-missing-terminal-proof",
        "retain-retirement-not-absent",
        "retain-failed-admission-cleanup",
        "retain-binding-refusal",
        "retain-fenced-completion-same-state",
    ] {
        retained_case(case).with_context(|| case.to_owned())?;
    }
    Ok(())
}

/// Executes production materialization against real Git, with only sandbox paths redirected.
fn source_fixture(reference: &str) -> Result<Value> {
    use std::path::PathBuf;
    use std::process::Command;
    struct GitPlane {
        root: PathBuf,
        agents: Cell<usize>,
    }
    impl session::InitPort for &GitPlane {
        async fn run(&self, argv: &[&str], _: bool) -> Result<String> {
            let output = Command::new(argv[0])
                .args(argv[1..].iter().map(|arg| {
                    arg.strip_prefix("/workspace").map_or_else(
                        || (*arg).to_owned(),
                        |suffix| format!("{}{suffix}", self.root.display()),
                    )
                }))
                .env_clear()
                .env("PATH", "/usr/bin:/bin")
                .env("HOME", &self.root)
                .env("GIT_CONFIG_NOSYSTEM", "1")
                .env("GIT_CONFIG_GLOBAL", "/dev/null")
                .output()?;
            anyhow::ensure!(
                output.status.success(),
                "Git fixture: {}",
                String::from_utf8_lossy(&output.stderr)
            );
            Ok(String::from_utf8(output.stdout)?.trim().to_owned())
        }
    }
    impl session::StartPort for &GitPlane {
        type Workspace = Self;
        fn worker_binding(&self) -> WorkerRecord {
            WorkerRecord {
                name: "default".into(),
                instance: "fixture".into(),
                region: "fixture".into(),
                data_volume: None,
            }
        }
        async fn create(&self, _: &str, _: &manifest::Resolved) -> Result<Self> {
            Ok(*self)
        }
        fn workspace_id<'a>(&self, _: &'a Self) -> &'a str {
            "workspace-1"
        }
        async fn start_agent(&self, _: &Self, _: &session::RunRequest, _: &str) -> Result<String> {
            self.agents.set(self.agents.get() + 1);
            Ok("exec-1".into())
        }
        async fn ready(&self, _: &Self, _: &str) -> Result<()> {
            Ok(())
        }
        async fn attach(
            &self,
            _: &Self,
            _: &str,
            _: &crate::domain::session::AgentKind,
        ) -> Result<()> {
            Ok(())
        }
    }
    let temporary = tempfile::tempdir()?;
    let origin = temporary.path().join("origin");
    std::fs::create_dir(&origin)?;
    let git = |args: &[&str]| -> Result<String> {
        let output = Command::new("/usr/bin/git")
            .arg("-C")
            .arg(&origin)
            .args([
                "-c",
                "user.name=b10x-bot[bot]",
                "-c",
                "user.email=b10x-bot[bot]@users.noreply.github.com",
                "-c",
                "commit.gpgsign=false",
                "-c",
                "tag.gpgsign=false",
            ])
            .args(args)
            .env_clear()
            .env("PATH", "/usr/bin:/bin")
            .env("HOME", temporary.path())
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .output()?;
        anyhow::ensure!(
            output.status.success(),
            "fixture setup: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        Ok(String::from_utf8(output.stdout)?.trim().to_owned())
    };
    git(&["init", "--initial-branch=main"])?;
    git(&["commit", "--allow-empty", "-m", "tagged commit"])?;
    let tagged = git(&["rev-parse", "HEAD"])?;
    git(&["tag", "lightweight"])?;
    git(&["tag", "-a", "annotated", "-m", "annotated release"])?;
    git(&["commit", "--allow-empty", "-m", "branch commit"])?;
    let branch = git(&["rev-parse", "HEAD"])?;
    let expected = if reference == "main" {
        &branch
    } else {
        &tagged
    };
    let declared = if reference == "commit" {
        &tagged
    } else {
        reference
    };
    let mut resolved = manifest::parse(
        "apiVersion: mantle.beyond10x.dev/v1alpha1\nkind: Session\nmetadata: {name: fixture}\nworkspace:\n  repositories:\n    - {name: r, repository: https://example.com/r.git, ref: main, mount: r}\nagent: {kind: claude-code, cwd: /workspace/r}\n",
    )?;
    resolved.repositories[0].repository = format!("file://{}", origin.display());
    resolved.repositories[0].reference = declared.to_owned();
    let plane = GitPlane {
        root: temporary.path().join("workspace"),
        agents: Cell::new(0),
    };
    let store = Store::in_memory()?;
    store.insert_session(&record(SessionState::Materializing))?;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    let result = runtime.block_on(session::start_recorded(
        &store,
        "session-1",
        &resolved,
        &&plane,
        false,
    ));
    let sources = store.sources("session-1")?;
    Ok(
        json!({"succeeded":result.is_ok(),"agent_started":plane.agents.get()==1,"source_count":sources.len(),"commit_matches":sources.first().is_some_and(|source|source.commit==*expected),"declared_matches":sources.first().is_some_and(|source|source.declared_ref==declared)}),
    )
}

#[test]
fn real_git_resolves_branches_tags_and_commits() {
    for reference in ["main", "lightweight", "annotated", "commit"] {
        assert_eq!(
            source_fixture(reference).unwrap(),
            json!({"succeeded":true,"agent_started":true,"source_count":1,"commit_matches":true,"declared_matches":true}),
            "{reference}"
        );
    }
    assert_eq!(
        source_fixture("missing").unwrap(),
        json!({"succeeded":false,"agent_started":false,"source_count":0,"commit_matches":false,"declared_matches":false})
    );
}
pub(super) fn execute(command: &str, input: &Value) -> Result<Reply> {
    if command == "mantle.orchestration.SourceResolution" {
        return Ok(Reply::returned(source_fixture(
            input["reference"].as_str().context("reference")?,
        )?));
    }
    if command == "mantle.orchestration.ExecOutcome" {
        let code: Value = serde_json::from_str(input["code_json"].as_str().context("code")?)?;
        let signal: Value = serde_json::from_str(input["signal_json"].as_str().context("signal")?)?;
        let stdout = input["stdout"].as_str().context("stdout")?;
        let stderr = input["stderr"].as_str().context("stderr")?;
        let output = serde_json::from_value::<b10x_substrate_sdk::RunOutput>(json!({
            "exec":{"id":"fixture","workspace":"ws","state":input["state"],"observed_at":"2026-10-03T00:00:00Z",
              "requested":{"capability_snapshot":"fixture","network":"none","profile":"workspace","require":true},"applied":null,"usage":null,"lease":null,
              "exit": if input["missing_exit"].as_bool()==Some(true) {Value::Null} else {json!({"code":code,"signal":signal})},
              "refusal":if input["refused"].as_bool()==Some(true) {json!({"class":"Failed","code":"fixture.refused","message":"fixture refusal"})} else {Value::Null}},
            "stdout":stdout.as_bytes(),"stderr":stderr.as_bytes(),"stdout_truncated":false,"stderr_truncated":false
        }));
        let mut actual_stdout = Vec::new();
        let mut actual_stderr = Vec::new();
        let result = output.map_err(anyhow::Error::from).and_then(|output| {
            session::write_exec_output(&output, &mut actual_stdout, &mut actual_stderr)
        });
        return Ok(Reply::returned(
            json!({"exit_code":result.as_ref().copied().unwrap_or(1),"known":result.is_ok(),"stdout":String::from_utf8(actual_stdout)?,"stderr_preserved":actual_stderr.starts_with(stderr.as_bytes())}),
        ));
    }
    if command == "mantle.orchestration.CodexPreflight" {
        return codex_preflight();
    }
    if command == "mantle.orchestration.Observe" {
        let facts: Value = serde_json::from_str(input["facts_json"].as_str().context("facts")?)?;
        let machine: b10x_substrate_sdk::Machine = serde_json::from_value(json!({
            "capability_snapshot":"snapshot-fixture", "driver_version":"0.7.8", "configuration_generation":1,
            "probed_at":"2026-10-02T00:00:00Z", "valid_until":null,"facts":facts,
            "guarded_workspace_io":true,"exec_argv_only":true,"exec_no_egress":true,"exec_cgroup_limits":true,"exec_cgroup_kill":true,"events_pull":true,"events_stream":true
        }))?;
        let usage: Option<b10x_substrate_sdk::ExecUsage> = input["usage_json"]
            .as_str()
            .map(serde_json::from_str)
            .transpose()?;
        let rows: Vec<_> = crate::adapters::substrate::describe_usage(usage.as_ref())
            .into_iter()
            .map(|(name, value)| json!({"name":name,"value":value}))
            .collect();
        // Absent selection preserves the original Claude capability contract. This boundary
        // observes capabilities only; the selected-agent setup separately probes its executable.
        let claude_selected = match &input["claude_selected"] {
            Value::Null => true,
            value => value.as_bool().context("claude_selected must be boolean")?,
        };
        return Ok(Reply::returned(
            json!({"missing":crate::adapters::substrate::missing_capability_facts(&machine, claude_selected),"quota_served":crate::adapters::substrate::quota_served(&machine),"usage":rows}),
        ));
    }
    if command == "mantle.orchestration.Request" {
        let request = match input["kind"].as_str().context("kind")? {
            "agent" => session::agent_request(&manifest::parse(
                input["manifest"].as_str().context("manifest")?,
            )?),
            "attach" => session::attach_request(&crate::domain::session::AgentKind::V0),
            _ => {
                let argv: Vec<String> = serde_json::from_value(input["argv"].clone())?;
                let (program, args) = argv.split_first().context("argv")?;
                session::exec_request(
                    program,
                    args,
                    mantle_conformance::integer(&input["cpu"])?.try_into()?,
                )
            }
        };
        let mut value = serde_json::to_value(request)?;
        let environment = value["environment"]
            .as_object()
            .context("environment")?
            .iter()
            .map(|(k, v)| json!({"name":k,"value":v}))
            .collect::<Vec<_>>();
        value["environment"] = json!(environment);
        return Ok(Reply::returned(value));
    }
    let store = Store::in_memory()?;
    let plane = Plane::new(input);
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    runtime.block_on(async {
        // Tokio's virtual clock drives the production five-minute read-back deadline exactly.
        tokio::time::pause();
        let result: Result<()> = match command {
            "mantle.orchestration.Start" => {
                let resolved = manifest::parse(input["manifest"].as_str().context("manifest")?)?;
                store.insert_session(&record(SessionState::Materializing))?;
                session::start_recorded(&store,"session-1",&resolved,&&plane,input["attach"].as_bool().unwrap_or(false)).await
            }
            "mantle.orchestration.Destroy" => {
                let mut r = record(plane.text("state").parse()?);
                r.workspace = input["has_workspace"].as_bool().unwrap_or(false).then(||"workspace-1".into());
                r.agent_exec = input["has_exec"].as_bool().unwrap_or(false).then(||"exec-1".into());
                store.insert_session(&r)?;
                crate::app::lifecycle::stop(&plane,&store,&r,true).await
            }
            "mantle.orchestration.Worker" => {
                let up = plane.text("action") == "up";
                let result = match (plane.text("provider"),up) {
                    ("aws",true) => worker::up_aws_with(&plane,"fixture","fixture",input["idle_stop"].as_bool().unwrap_or(false)).await.map(Some),
                    ("aws",false) => worker::down_aws_with(&plane).await.map(|()|None),
                    (_,true) => worker::up_kubevirt_with(&plane,"fixture").await.map(Some),
                    (_,false) => worker::down_kubevirt_with(&plane).await.map(|()|None),
                };
                return Ok(Reply::returned(json!({"succeeded":result.is_ok(),"calls":plane.calls.borrow().clone(),"instance":result.as_ref().ok().and_then(|r|r.as_ref().map(|r|&r.0)),"volume":result.as_ref().ok().and_then(|r|r.as_ref().and_then(|r|r.1.as_ref()))})));
            }
            "mantle.orchestration.WorkerRecords" => {
                for value in input["records"].as_array().context("records")? {
                    store.put_worker(&WorkerRecord{name:value["name"].as_str().context("name")?.into(), instance:value["instance"].as_str().context("instance")?.into(),region:value["region"].as_str().context("region")?.into(),data_volume:value["data_volume"].as_str().map(str::to_owned)})?;
                }
                let r=store.worker(plane.text("name"))?;
                return Ok(Reply::returned(json!({"record":r.map(|r|json!({"name":r.name,"instance":r.instance,"region":r.region,"data_volume":r.data_volume}))})));
            }
            _ => bail!("unsupported orchestration command {command}"),
        };
        let r=store.session_by_id("session-1")?.context("record disappeared")?;
        let sources:Vec<_>=store.sources("session-1")?.iter().map(|s|json!({"mount":s.mount,"commit":s.commit,"declared_ref":s.declared_ref})).collect();
        Ok(Reply::returned(json!({"succeeded":result.is_ok(),"state":r.state.to_string(),"workspace":r.workspace,"agent_exec":r.agent_exec,"failure":r.failure,"calls":plane.calls.borrow().clone(),"sources":sources})))
    })
}

fn codex_preflight() -> Result<Reply> {
    let resolved = manifest::parse(include_str!("../../../../examples/codex.yaml"))?;
    let store = Store::in_memory()?;
    let reads = std::cell::Cell::new(0);
    let credentials = session::selected_credentials(&resolved, || {
        reads.set(reads.get() + 1);
        Ok(b"synthetic-claude-token".to_vec())
    });
    let plane = Plane::new(&json!({"commit":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}));
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    let mut persisted = record(SessionState::Materializing);
    persisted.agent_kind = resolved.agent_kind.clone();
    persisted.authentication = resolved.authentication.clone();
    store.insert_session(&persisted)?;
    let started = runtime.block_on(session::start_recorded(
        &store,
        &persisted.id,
        &resolved,
        &&plane,
        true,
    ));
    let stored = store
        .live_session(&persisted.name)?
        .context("stored selection")?;
    let attach = serde_json::to_value(session::attach_request(&stored.agent_kind))?;
    let request = serde_json::to_value(session::agent_request(&resolved))?;
    Ok(Reply::returned(json!({
        "started": credentials.is_ok_and(|value| value.is_none()) && started.is_ok(),
        "credential_reads":reads.get(), "calls":plane.calls.borrow().clone(),
        "state":stored.state.to_string(), "agent_exec":stored.agent_exec,
        "attach_argv":attach["argv"],
        "attach_has_secret":!attach["secret_slot"].is_null() || !attach["secret_fd"].is_null(),
        "request_has_secret":!request["secret_slot"].is_null() || !request["secret_fd"].is_null()
    })))
}

/// Native release scenarios run the production verifier and installer over real ELF fixtures.
fn release(command: &str, input: &Value) -> Result<Reply> {
    use mantle_artifact::{GNU, MUSL, Manifest};
    use std::{collections::BTreeMap, fs, process::Command, sync::OnceLock};
    static BINARIES: OnceLock<(tempfile::TempDir, Vec<u8>, Vec<u8>)> = OnceLock::new();
    let binaries = BINARIES.get_or_init(|| {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("fixture.rs");
        fs::write(&source, "fn main() { println!(\"release fixture\"); }\n").unwrap();
        let compile = |target: &str| {
            let out = dir.path().join(target);
            let output = mantle_worker::run_bounded(
                Command::new("rustc")
                    .arg(&source)
                    .args(["--target", target, "-o"])
                    .arg(&out),
                None,
                Duration::from_secs(30),
                1024 * 1024,
            )
            .unwrap();
            assert!(output.status.success());
            fs::read(out).unwrap()
        };
        let gnu = compile(GNU);
        let musl = compile(MUSL);
        (dir, gnu, musl)
    });
    let temp = tempfile::tempdir()?;
    let source = "1111111111111111111111111111111111111111";
    let mut manifest = Manifest {
        format: "mantle-release/1".into(),
        version: "0.1.4".into(),
        source_commit: source.into(),
        substrate_revision: "65304edf6ebdf4a95f9c2c6138b0c20ea47d157e".into(),
        substrate_version: "0.7.10".into(),
        rustc: "rustc fixture".into(),
        gnu_runtime: "glibc 2.42".into(),
        musl_runtime: "1.2.5".into(),
        artifacts: Vec::new(),
    };
    for (target, binary) in [(GNU, &binaries.1), (MUSL, &binaries.2)] {
        let files: BTreeMap<_, _> = mantle_artifact::inventory(target)?
            .keys()
            .map(|path| {
                (
                    path.clone(),
                    if path.starts_with("bin/") {
                        binary.clone()
                    } else {
                        b"fixture notice\n".to_vec()
                    },
                )
            })
            .collect();
        let (artifact, bytes) = mantle_release::build::archive(&files, target, "0.1.4")?;
        fs::write(temp.path().join(&artifact.name), bytes)?;
        manifest.artifacts.push(artifact);
    }
    match input["alteration"].as_str().unwrap_or("none") {
        "none" => {}
        "source" => manifest.source_commit = "invalid".into(),
        "checksum" => {
            let artifact = &manifest.artifacts[0];
            let path = temp.path().join(&artifact.name);
            let mut bytes = fs::read(&path)?;
            // Change TAR padding, preserving length, member metadata and every payload digest.
            // A truncated archive only proves the independent size refusal.
            let first = &artifact.payloads[0];
            anyhow::ensure!(first.path == "LICENSE" && !first.size_bytes.is_multiple_of(512));
            let padding = 512 + usize::try_from(first.size_bytes)?;
            bytes[padding] ^= 1;
            fs::write(path, bytes)?;
        }
        "target" => manifest.artifacts[0].target = "unsupported".into(),
        "path" => manifest.artifacts[0].payloads[0].path = "../outside".into(),
        other => bail!("unknown release fixture alteration {other}"),
    }
    let path = temp.path().join("manifest.json");
    fs::write(&path, serde_json::to_vec(&manifest)?)?;
    let verified = mantle_artifact::verify(&path);
    if command == "mantle.operator.InstallRelease" {
        let prefix = temp.path().join("prefix");
        fs::create_dir(&prefix)?;
        fs::create_dir(prefix.join("bin"))?;
        if input["collision"].as_bool().unwrap_or(true) {
            fs::write(prefix.join("bin/mantle"), b"previous unmanaged binary")?;
        }
        let outcome = mantle_release::install(&verified?, &prefix, GNU);
        return Ok(Reply::returned(
            json!({"accepted":outcome.is_ok(),"previous_preserved":fs::read(prefix.join("bin/mantle")).is_ok_and(|b|b==b"previous unmanaged binary")}),
        ));
    }
    let payload_count = verified
        .as_ref()
        .map(|b| b.artifacts.values().map(|v| v.len()).sum::<usize>())
        .unwrap_or(0);
    Ok(Reply::returned(
        json!({"accepted":verified.is_ok(),"payload_count":payload_count,"source_matches":verified.as_ref().is_ok_and(|b|b.manifest.source_commit==source)}),
    ))
}

fn acceptance(input: &Value) -> Result<Reply> {
    use mantle_acceptance::runner::{
        Checkpoint, Evidence, INVENTORY, attest, inventory, invoke, validate_receipt,
    };
    use mantle_acceptance::{CreationReceipt, SelectionBinding, Session, encoded};
    let selection = SelectionBinding {
        profile: Some("fixture".into()),
        config_path: "/fixture/config".into(),
        state_dir: "/fixture/state".into(),
        config_sha256: "a".repeat(64),
    };
    let session = Session {
        id: "owned".into(),
        name: "fixture".into(),
        agent: "codex".into(),
        authentication: "chatgpt-device".into(),
        recorded_state: "RUNNING".into(),
        generation: 0,
        workspace_id: Some("ws".into()),
        exec_id: Some("exec".into()),
        source_commits: vec![],
        observed: None,
    };
    let mut run = Checkpoint {
        format: "mantle-acceptance/v1".into(),
        run_id: "run".into(),
        agent: "codex".into(),
        profile: "fixture".into(),
        selection: selection.clone(),
        executable: "/fixture/mantle".into(),
        executable_sha256: "b".repeat(64),
        executable_version: "mantle 0.1.4".into(),
        source_commit: None,
        run_dir: "/fixture/run".into(),
        phase: "manual".into(),
        session: Some(session.clone()),
        results: INVENTORY
            .iter()
            .map(|case| Evidence {
                case: (*case).into(),
                status: if matches!(*case, "login" | "model-tool" | "visual") {
                    "operator-required"
                } else {
                    "passed"
                }
                .into(),
                origin: if matches!(*case, "login" | "model-tool" | "visual") {
                    "operator"
                } else {
                    "machine"
                }
                .into(),
                phase: "initial".into(),
                exec_id: Some("exec".into()),
                observed_at: "time".into(),
            })
            .collect(),
    };
    let accepted = match input["case"].as_str().unwrap_or("inventory") {
        "inventory" => {
            inventory("codex")? == inventory("claude-code")? && inventory("other").is_err()
        }
        "command-path" => {
            let directory = tempfile::tempdir()?;
            let binary = directory.path().join("cli");
            let config = b"synthetic config";
            std::fs::write(directory.path().join("config.toml"), config)?;
            std::fs::write(
                directory.path().join("manifest.yaml"),
                "metadata:\n  name: fixture\nagent:\n  kind: codex\n",
            )?;
            let output = invoke(
                std::process::Command::new("rustc")
                    .args([
                        "--edition=2024",
                        concat!(
                            env!("CARGO_MANIFEST_DIR"),
                            "/../mantle-acceptance/tests/support/cli.rs"
                        ),
                        "-o",
                    ])
                    .arg(&binary)
                    .env("ACCEPTANCE_BUILD_ROOT", directory.path())
                    .env("ACCEPTANCE_BUILD_SHA", mantle_artifact::sha256(config)),
                Duration::from_secs(10),
            )?;
            anyhow::ensure!(
                output.status.success(),
                "acceptance CLI fixture failed compilation"
            );
            let checkpoint = directory.path().join("checkpoint.json");
            let status = mantle_acceptance::runner::run(
                &binary,
                "fixture",
                "codex",
                &directory.path().join("manifest.yaml"),
                &checkpoint,
                None,
            )?;
            let saved: Checkpoint = serde_json::from_slice(&std::fs::read(checkpoint)?)?;
            status == 2 && saved.session.as_ref().is_some_and(|s| s.id == "owned")
        }
        "auth-required" => {
            run.exit_code() == 2
                && run
                    .results
                    .iter()
                    .filter(|r| r.status == "operator-required")
                    .count()
                    == 3
        }
        "no-transcript" => {
            let mut data = serde_json::to_value(&run)?;
            data["raw_terminal"] = json!("untrusted transcript");
            serde_json::from_value::<Checkpoint>(data).is_err()
                && !String::from_utf8(encoded(&run)?)?.contains("raw_terminal")
        }
        "failure-exit" => {
            run.results[0].status = "failed".into();
            run.exit_code() == 1
        }
        "resume" => {
            let stale = attest(&mut run, "wrong", "exec", &["login"]).is_err();
            attest(&mut run, "run", "exec", &["login", "model-tool", "visual"])?;
            stale && run.exit_code() == 0
        }
        "owned-cleanup" => {
            let mut legacy = selection.clone();
            legacy.profile = None;
            let mut receipt = CreationReceipt {
                format: mantle_acceptance::RECEIPT_FORMAT.into(),
                selection: legacy,
                session,
                created_at: "time".into(),
            };
            let own = validate_receipt(&receipt, &selection, "fixture", "codex").is_ok();
            receipt.session.name = "unrelated".into();
            own && validate_receipt(&receipt, &selection, "fixture", "codex").is_err()
        }
        "no-fabricated-pass" => {
            attest(&mut run, "run", "exec", &["login"])?;
            run.exit_code() == 2
        }
        other => bail!("unknown acceptance case {other}"),
    };
    let directory = tempfile::tempdir()?;
    mantle_acceptance::write_new(&directory.path().join("checkpoint"), &run)?;
    use std::os::unix::fs::PermissionsExt;
    let private = std::fs::metadata(directory.path().join("checkpoint"))?
        .permissions()
        .mode()
        & 0o077
        == 0;
    Ok(Reply::returned(
        json!({"accepted":accepted,"complete":run.exit_code()==0,"count":run.results.len(),"private":private}),
    ))
}
