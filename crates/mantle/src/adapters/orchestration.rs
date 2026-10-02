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

struct Plane {
    input: Value,
    calls: RefCell<Vec<String>>,
    reads: Cell<usize>,
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
    async fn create(&self, _: &str, _: &manifest::Resolved) -> Result<Self> {
        self.call("create")?;
        Ok(*self)
    }
    fn workspace_id<'a>(&self, _: &'a Self) -> &'a str {
        "workspace-1"
    }
    async fn start_agent(&self, _: &Self, _: &manifest::Resolved) -> Result<String> {
        self.call("agent")?;
        Ok("exec-1".into())
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
    async fn signal(&self, _: &mut ExecState) -> Result<()> {
        self.call("signal")
    }
    async fn wait(&self, _: &mut ExecState) -> Result<()> {
        self.call("wait")
    }
    async fn retire(&self, _: ExecState) -> Result<()> {
        self.call("retire")
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
pub(super) fn execute(command: &str, input: &Value) -> Result<Reply> {
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
            "mantle.orchestration.Stop" => {
                let mut r = record(plane.text("state").parse()?);
                r.workspace = input["has_workspace"].as_bool().unwrap_or(false).then(||"workspace-1".into());
                r.agent_exec = input["has_exec"].as_bool().unwrap_or(false).then(||"exec-1".into());
                store.insert_session(&r)?;
                session::stop_recorded(&plane,&store,&r,"fixture").await
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
    let result = session::selected_credentials(&resolved, || {
        reads.set(reads.get() + 1);
        Ok(b"synthetic-claude-token".to_vec())
    });
    let plane = Plane::new(&json!({}));
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    // This is the real start flow with its admission check, not a fake successful provider.
    let started = runtime.block_on(session::start_recorded(
        &store, "blocked", &resolved, &&plane, true,
    ));
    let rows = store.live_sessions()?.len();
    let mut persisted = record(SessionState::Running);
    persisted.agent_kind = resolved.agent_kind.clone();
    persisted.authentication = resolved.authentication.clone();
    store.insert_session(&persisted)?;
    let stored = store
        .live_session(&persisted.name)?
        .context("stored selection")?;
    let attach = session::attach_request(&stored.agent_kind);
    let request = session::agent_request(&resolved);
    Ok(Reply::returned(
        json!({"refused":result.is_err() && started.is_err(),
        "diagnostic":result.unwrap_err().to_string(),"credential_reads":reads.get(),
        "calls":plane.calls.borrow().clone(),"session_rows":rows,
        "attach_requires_non_recording":attach.requires_non_recording,
        "request_requires_non_recording":request.requires_non_recording}),
    ))
}
