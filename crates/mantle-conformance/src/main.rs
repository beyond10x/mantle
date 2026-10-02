//! Reconcile native component runs against the complete ESS inventory before publishing evidence.
use anyhow::{Context, Result, ensure};
use clap::Parser;
use ess_conformance::{
    AdmittedSuite,
    counts::{CountReport, CountRun},
    results::ExternalResults,
};
use serde_json::{Value, json};
use std::{collections::BTreeSet, path::PathBuf, process::Command};

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = ".")]
    root: PathBuf,
}
fn read(path: impl AsRef<std::path::Path>) -> Result<Value> {
    Ok(serde_json::from_slice(&std::fs::read(path)?)?)
}
fn main() -> Result<()> {
    let root = Args::parse().root;
    reject_unmapped(&root.join("spec"))?;
    let drafts = root.join(".engineering/drafts");
    let full_path = drafts.join("mantle-suite.json");
    let output = Command::new("ess")
        .args(["verify", "conform", "synthesize", "--path"])
        .arg(root.join("spec"))
        .arg("--scenarios")
        .arg(root.join("spec"))
        .args(["--suite-format", "5", "--out"])
        .arg(&full_path)
        .output()?;
    ensure!(
        output.status.success(),
        "ESS synthesis: {}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    std::fs::write(drafts.join("mantle-synthesis.txt"), &output.stdout)?;
    ensure!(
        !String::from_utf8_lossy(&output.stdout)
            .lines()
            .any(|s| s.starts_with("refused")),
        "ESS synthesis has refusals"
    );
    let full = AdmittedSuite::from_json(&std::fs::read_to_string(&full_path)?)?;
    let full_json = read(&full_path)?;
    std::fs::write(
        drafts.join("mantle-noop-audit.json"),
        serde_json::to_vec_pretty(&mantle_conformance::audit_noop(&full)?)?,
    )?;
    let baseline = read(root.join("spec/conformance-baseline.json"))?;
    let mut seen = BTreeSet::new();
    let mut results = Vec::new();
    let mut completed_at = 0;
    for component in ["mantle-cli", "mantle-egress", "mantle-launch"] {
        let suite_path = drafts.join(format!("{component}-suite.json"));
        let suite = AdmittedSuite::from_json(&std::fs::read_to_string(&suite_path)?)?;
        let suite_json = read(&suite_path)?;
        ensure!(
            suite_json["provenance"]["spec_digest"] == full_json["provenance"]["spec_digest"],
            "stale component {component}"
        );
        let run_path = drafts.join(format!("{component}-run.json"));
        CountRun::from_json(&std::fs::read_to_string(&run_path)?, &suite)?;
        let run = read(&run_path)?;
        let report = CountReport::from_json(
            &std::fs::read_to_string(drafts.join(format!("{component}-report.json")))?,
            &suite,
        )?;
        ensure!(
            run["summary"] == serde_json::to_value(&report)?,
            "component report/run mismatch"
        );
        completed_at = completed_at.max(report.completed_at());
        let counts = report.counts();
        let floor = baseline["components"][component]["answered_floor"]
            .as_u64()
            .context("component baseline")?;
        ensure!(
            counts.passed + counts.failed >= floor && counts.total >= floor,
            "{component}: coverage floor {floor}"
        );
        ensure!(
            counts.failed == 0
                && counts.error == 0
                && counts.unsupported == 0
                && counts.skipped == 0,
            "{component}: incomplete run"
        );
        for result in run["scenarios"].as_array().context("scenarios")? {
            let id = result["scenario"].as_str().context("scenario id")?;
            ensure!(seen.insert(id.to_owned()), "duplicate scenario {id}");
            ensure!(
                full_json["scenarios"].get(id) == suite_json["scenarios"].get(id),
                "{id}: executed scenario differs from full suite"
            );
            ensure!(
                !result["checks"].as_array().context("checks")?.is_empty(),
                "{id}: no checks"
            );
            results.push(json!({"scenario_id":id,"status":result["status"]}));
        }
    }
    for id in baseline["required_scenarios"]
        .as_array()
        .context("required scenarios")?
    {
        ensure!(
            seen.contains(id.as_str().context("baseline id")?),
            "required scenario disappeared: {id}"
        );
    }
    // ESS itself rejects missing, additional and duplicate results. This is an external aggregation
    // of real Rust runs, not a claim that ESS executed one combined process.
    let raw = json!({"format":"ess-conformance-results/1","suite_digest":full.digest(),"completed_at":completed_at,"results":results});
    let external = ExternalResults::from_json(&raw.to_string(), &full)?;
    let report = CountReport::from_external(
        &external,
        &full,
        "mantle-native component aggregation 0.1.0",
        None,
    )?;
    let value = serde_json::to_value(&report)?;
    ensure!(
        value["conformance_status"] == "passed"
            && value["coverage"]["counts"]["outside"] == 0
            && value["coverage"]["counts"]["refused"] == 0,
        "full coverage is incomplete"
    );
    std::fs::write(
        drafts.join("mantle-results.json"),
        serde_json::to_vec_pretty(&raw)?,
    )?;
    std::fs::write(
        drafts.join("mantle-report.json"),
        report.to_canonical_json()?,
    )?;
    println!(
        "Complete ESS inventory: {} passed; 0 failed, skipped, unsupported, outside or refused",
        report.counts().passed
    );
    Ok(())
}

fn reject_unmapped(path: &std::path::Path) -> Result<()> {
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            reject_unmapped(&entry.path())?;
        } else if matches!(
            entry.path().extension().and_then(|e| e.to_str()),
            Some("yaml" | "yml" | "md" | "json")
        ) {
            ensure!(
                !std::fs::read_to_string(entry.path())?.contains("UNMAPPED"),
                "open specification marker in {}",
                entry.path().display()
            );
        }
    }
    Ok(())
}
