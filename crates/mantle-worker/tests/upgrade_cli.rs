use std::process::Command;

#[test]
fn offline_upgrade_commands_are_available_without_agent_initialization() {
    for command in ["upgrade-check", "upgrade-apply", "reconcile-helpers"] {
        let output = Command::new(env!("CARGO_BIN_EXE_mantle-worker"))
            .args([command, "--help"])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{command}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
