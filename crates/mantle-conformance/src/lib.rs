//! Adapts production observations to the pinned ESS Rust runner. No model or predicate evaluator.
use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

use anyhow::{Result, bail, ensure};
use base64::Engine as _;
use ess_conformance::runner::{Clock, Ids, RunnerConfig};
use ess_conformance::target::*;
use ess_conformance::{AdmittedSuite, Runner, counts, report::Status};
use ess_primitives::{consistency::ConsistencyToken, time::Timestamp};
use ess_primitives::{facts::Number, node::Node};
use serde_json::Value;

pub fn bytes(value: &Value) -> Result<Vec<u8>> {
    Ok(base64::engine::general_purpose::STANDARD.decode(
        value
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("expected base64 Bytes"))?,
    )?)
}
pub fn encoded(bytes: &[u8]) -> Value {
    Value::String(base64::engine::general_purpose::STANDARD.encode(bytes))
}

#[derive(Default)]
pub struct Reply {
    pub outcome: String,
    pub error: Option<String>,
    pub events: BTreeMap<String, Value>,
    pub response: Option<Value>,
}

impl Reply {
    pub fn returned(response: Value) -> Self {
        Self {
            outcome: "returned".into(),
            response: Some(response),
            ..Self::default()
        }
    }
}

pub trait Boundary: Default {
    fn execute(&mut self, command: &str, input: &Value) -> Result<Reply>;
    fn external(&mut self, outcome: &str) -> Result<()> {
        bail!("unmapped external outcome {outcome}")
    }
    fn query(&self, view: &str) -> Result<Value> {
        bail!("unmapped view {view}")
    }
}

pub fn integer(value: &Value) -> Result<i128> {
    if let Some(v) = value.as_i64() {
        return Ok(i128::from(v));
    }
    if let Some(v) = value.as_u64() {
        return Ok(i128::from(v));
    }
    if let Some(v) = value.as_f64() {
        ensure!(
            v.fract() == 0.0 && v.abs() <= 9_007_199_254_740_992.0,
            "inexact integer {v}"
        );
        return Ok(v as i128);
    }
    bail!("expected Integer, got {value}")
}

// Preserve unsigned widths and integers above 2^53. Never pass Integer through f64.
fn node(value: Value) -> Result<Node> {
    Ok(match value {
        Value::Null => Node::Null,
        Value::Bool(v) => Node::Bool(v),
        Value::String(v) => Node::Text(v),
        Value::Number(v) => {
            if let Some(v) = v.as_i64() {
                Node::Number(Number::from(v))
            } else if v.as_u64().is_some() {
                // ESS's canonical reader retains unsigned integer tokens exactly; round trips
                // at u64::MAX and above the binary64 exact range are held by the test below.
                serde_json::from_str(&v.to_string())?
            } else {
                bail!("non-integer boundary number {v}")
            }
        }
        Value::Array(v) => Node::Seq(v.into_iter().map(node).collect::<Result<_>>()?),
        Value::Object(v) => Node::Map(
            v.into_iter()
                .map(|(k, v)| Ok((k, node(v)?)))
                .collect::<Result<_>>()?,
        ),
    })
}

fn fields(value: Value) -> Result<BTreeMap<String, Node>> {
    match node(value)? {
        Node::Map(fields) => Ok(fields),
        _ => bail!("expected an object observation"),
    }
}

fn unavailable(error: anyhow::Error) -> TargetError {
    TargetError::unavailable("production boundary", format!("{error:#}"))
}

struct Target<B>(RefCell<B>, Cell<u64>);
impl<B: Boundary> ConformanceTarget for Target<B> {
    fn identity(&self) -> Result<ImplementationIdentity, TargetError> {
        Ok(ImplementationIdentity::new(
            "mantle-native",
            env!("CARGO_PKG_VERSION"),
        ))
    }
    fn begin_scenario(&self, _: &ScenarioContext) -> Result<(), TargetError> {
        *self.0.borrow_mut() = B::default();
        self.1.set(0);
        Ok(())
    }
    fn end_scenario(&self, _: &ScenarioContext) -> Result<(), TargetError> {
        Ok(())
    }
    fn execute_command(
        &self,
        request: SemanticCommandRequest,
    ) -> Result<SemanticCommandResult, TargetError> {
        let input = serde_json::to_value(&request.input).map_err(|e| unavailable(e.into()))?;
        let reply = self
            .0
            .borrow_mut()
            .execute(&request.command.to_string(), &input)
            .map_err(unavailable)?;
        let mut result = SemanticCommandResult::took(ess_conformance::scenario::OutcomeRef::new(
            request.command,
            reply
                .outcome
                .parse()
                .map_err(|e| TargetError::unavailable("outcome", format!("{e}")))?,
        ));
        if let Some(error) = reply.error {
            result.error =
                Some(DeclaredErrorValue::new(error.parse().map_err(|e| {
                    TargetError::unavailable("error", format!("{e}"))
                })?));
        }
        for (event, payload) in reply.events {
            let mut observed = ObservedEvent::new(
                event
                    .parse()
                    .map_err(|e| TargetError::unavailable("event", format!("{e}")))?,
            );
            observed.payload = fields(payload).map_err(unavailable)?;
            result.direct_events.push(observed);
        }
        result.response = reply
            .response
            .map(fields)
            .transpose()
            .map_err(unavailable)?;
        // The boundary completes its synchronous writes or awaited IO before replying.
        // A read uses that same boundary after the acknowledged operation.
        self.1.set(self.1.get() + 1);
        result.consistency = Some(
            ConsistencyToken::new(self.1.get().to_string()).map_err(|e| unavailable(e.into()))?,
        );
        Ok(result)
    }
    fn query_view(&self, request: SemanticViewRequest) -> Result<SemanticViewResult, TargetError> {
        if let Some(token) = request.consistency.token() {
            let committed = token
                .as_str()
                .parse::<u64>()
                .map_err(|e| unavailable(e.into()))?;
            if committed > self.1.get() {
                return Err(TargetError::unavailable(
                    "read-your-writes",
                    "unacknowledged operation",
                ));
            }
        }
        if !request.params.is_empty() {
            return Err(TargetError::unsupported(
                "view parameters",
                request.view.to_string(),
            ));
        }
        let value = self
            .0
            .borrow()
            .query(&request.view.to_string())
            .map_err(unavailable)?;
        let rows = value
            .as_array()
            .ok_or_else(|| TargetError::unavailable("view", "expected rows"))?;
        Ok(SemanticViewResult::of(
            rows.iter()
                .cloned()
                .map(fields)
                .collect::<Result<Vec<_>>>()
                .map_err(unavailable)?,
        ))
    }
    fn observe_events(
        &self,
        _: EventObservationRequest,
    ) -> Result<Vec<ObservedEvent>, TargetError> {
        Err(TargetError::unsupported(
            "asynchronous events",
            "Mantle has no event bus",
        ))
    }
    fn configure_external_outcome(
        &self,
        control: ExternalOutcomeControl,
    ) -> Result<(), TargetError> {
        self.0
            .borrow_mut()
            .external(&control.force.to_string())
            .map_err(unavailable)
    }
    fn redeliver_event(&self, _: RedeliveryRequest) -> Result<(), TargetError> {
        Err(TargetError::unsupported(
            "redelivery",
            "Mantle has no event bus",
        ))
    }
}

pub fn run<B: Boundary>(component: &str, minimum: usize) -> Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let drafts = root.join(".engineering/drafts");
    std::fs::create_dir_all(&drafts)?;
    let suite_path = drafts.join(format!("{component}-suite.json"));
    let output = Command::new("ess")
        .current_dir(&root)
        .args([
            "verify",
            "conform",
            "synthesize",
            "--path",
            "spec",
            "--scenarios",
            &format!(
                "spec/scenarios/{}",
                component.strip_prefix("mantle-").expect("component prefix")
            ),
            "--suite-format",
            "5",
            "--component",
            component,
            "--out",
        ])
        .arg(&suite_path)
        .output()?;
    ensure!(
        output.status.success(),
        "ESS: {}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let synthesis = String::from_utf8(output.stdout)?;
    eprintln!("{synthesis}");
    ensure!(
        !synthesis.lines().any(|line| line.starts_with("refused")),
        "synthesis is incomplete"
    );
    let bytes = std::fs::read_to_string(&suite_path)?;
    let admitted = AdmittedSuite::from_json(&bytes)?;
    let target = Target(RefCell::new(B::default()), Cell::new(0));
    let run = Runner::new(
        RunnerConfig::default(),
        WallClock,
        Ids::for_suite(admitted.suite()),
    )
    .run_admitted(&admitted, &target);
    let report = counts::CountReport::from_run(&run, &admitted)?;
    std::fs::write(
        drafts.join(format!("{component}-report.json")),
        report.to_canonical_json()?,
    )?;
    std::fs::write(
        drafts.join(format!("{component}-run.json")),
        counts::CountRun::from_run(&run, &admitted)?.to_canonical_json()?,
    )?;
    ensure!(
        run.scenarios.len() >= minimum,
        "conformance scenario floor {minimum}"
    );
    let failures: Vec<_> = run
        .scenarios
        .iter()
        .filter(|r| r.status != Status::Passed)
        .collect();
    ensure!(failures.is_empty(), "ESS failures: {failures:#?}");
    eprintln!(
        "{component}: {} passed, 0 failed, 0 unsupported",
        run.scenarios.len()
    );
    Ok(())
}

struct WallClock;
impl Clock for WallClock {
    fn now(&mut self) -> Timestamp {
        Timestamp::from_epoch_millis(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("UTC clock")
                .as_millis()
                .try_into()
                .expect("millisecond clock fits u64"),
        )
    }
}

/// A deliberately inert target audits whether authored scenarios witness a consequence.
/// Its accepted commands never write state, emit events, or return observations.
pub fn audit_noop(admitted: &AdmittedSuite) -> Result<Value> {
    #[derive(Default)]
    struct Noop;
    impl Boundary for Noop {
        fn execute(&mut self, command: &str, _: &Value) -> Result<Reply> {
            let outcome = match command {
                "mantle.session.InsertSession" => "recorded",
                "mantle.session.SetWorkspace" | "mantle.session.SetAgentExec" => "recorded",
                "mantle.session.Materialized"
                | "mantle.session.MaterializationFailed"
                | "mantle.session.AgentStarted"
                | "mantle.session.AgentStartFailed"
                | "mantle.session.BeginStop"
                | "mantle.session.FinishStop" => "moved",
                "mantle.manifest.ValidateMemory"
                | "mantle.manifest.ValidatePids"
                | "mantle.manifest.ValidateRetention"
                | "mantle.launch.ValidateScrollback" => "accepted",
                "mantle.egress.CheckDefaultDestination" => "allowed",
                _ => "returned",
            };
            Ok(Reply {
                outcome: outcome.into(),
                response: Some(serde_json::json!({})),
                ..Reply::default()
            })
        }
        fn query(&self, _: &str) -> Result<Value> {
            Ok(serde_json::json!([]))
        }
        fn external(&mut self, _: &str) -> Result<()> {
            Ok(())
        }
    }
    let run = Runner::new(
        RunnerConfig::default(),
        WallClock,
        Ids::for_suite(admitted.suite()),
    )
    .run_admitted(admitted, &Target(RefCell::new(Noop), Cell::new(0)));
    let authored_passes: Vec<_> = run
        .scenarios
        .iter()
        .filter(|s| s.scenario.to_string().contains("/authored/") && s.status == Status::Passed)
        .map(|s| s.scenario.to_string())
        .collect();
    ensure!(
        authored_passes.is_empty(),
        "authored scenarios passed a no-op target: {authored_passes:?}"
    );
    Ok(
        serde_json::json!({"format":"mantle-noop-audit/1","implementation":"deliberately inert accepted commands and empty observations","spec_digest":admitted.suite().provenance.spec_digest,"scenarios":run.scenarios.len(),"authored_passes":authored_passes,"passed":run.scenarios.iter().filter(|s|s.status==Status::Passed).map(|s|s.scenario.to_string()).collect::<Vec<_>>()}),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn observations_preserve_full_integer_width() -> Result<()> {
        for number in [9_007_199_254_740_993, u64::MAX] {
            let value = serde_json::json!(number);
            ensure!(serde_json::to_value(node(value.clone())?)? == value);
        }
        Ok(())
    }
}
