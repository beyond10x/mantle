//! Deterministic installed-CLI boundary fixture. No provider, credentials, or real sessions.
use std::{
    fs,
    io::{Read, Write},
    os::unix::fs::PermissionsExt,
    path::PathBuf,
};
fn quoted(s: &str) -> String {
    format!(
        "\"{}\"",
        s.replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
    )
}
fn main() {
    let root = PathBuf::from(
        std::env::var_os("ACCEPTANCE_FIXTURE_ROOT")
            .or_else(|| option_env!("ACCEPTANCE_BUILD_ROOT").map(Into::into))
            .unwrap(),
    );
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("--version") {
        println!("mantle 0.1.4");
        return;
    }
    let verb_index = if args.first().map(String::as_str) == Some("--profile") {
        2
    } else {
        0
    };
    let verb = &args[verb_index];
    let profile = if verb_index == 2 {
        quoted(&args[1])
    } else {
        "null".into()
    };
    let selection = format!(
        "{{\"profile\":{profile},\"config_path\":{},\"state_dir\":{},\"config_sha256\":{}}}",
        quoted(root.join("config.toml").to_str().unwrap()),
        quoted(root.join("state").to_str().unwrap()),
        quoted(
            &std::env::var("ACCEPTANCE_CONFIG_SHA").unwrap_or_else(|_| option_env!(
                "ACCEPTANCE_BUILD_SHA"
            )
            .unwrap()
            .into())
        )
    );
    let mode = std::env::var("ACCEPTANCE_FIXTURE_MODE").unwrap_or_default();
    let session_path = root.join("session");
    let session = || {
        let data = fs::read_to_string(&session_path).unwrap();
        let fields: Vec<_> = data.split('|').collect();
        format!(
            "{{\"id\":\"owned\",\"name\":{},\"agent\":{},\"authentication\":{},\"recorded_state\":{},\"generation\":0,\"workspace_id\":\"ws\",\"exec_id\":{},\"source_commits\":[\"{}\"],\"observed\":{{\"workspace_state\":\"ready\",\"exec_state\":\"running\",\"exit_code\":null,\"exit_signal\":null,\"refused\":false}}}}",
            quoted(fields[0]),
            quoted(fields[1]),
            quoted(if fields[1] == "codex" {
                "chatgpt-device"
            } else {
                "claude-oauth"
            }),
            quoted(fields[3]),
            quoted(fields[2]),
            "a".repeat(40)
        )
    };
    fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(root.join("calls"))
        .unwrap()
        .write_all(format!("{}\n", args.join(" ")).as_bytes())
        .unwrap();
    match verb.as_str() {
        "list" => println!(
            "{{\"format\":\"mantle-session-metadata/v1\",\"selection\":{selection},\"outcome\":\"recorded\",\"observed_at\":\"time\",\"sessions\":[]}}"
        ),
        "start" => {
            let manifest = fs::read_to_string(&args[verb_index + 1]).unwrap();
            let name = manifest
                .lines()
                .find_map(|l| l.trim().strip_prefix("name: "))
                .unwrap();
            let agent = if manifest.contains("kind: codex") {
                "codex"
            } else {
                "claude-code"
            };
            fs::write(&session_path, format!("{name}|{agent}|exec1|RUNNING")).unwrap();
            if mode == "lost-receipt" {
                return;
            }
            let receipt =
                PathBuf::from(&args[args.iter().position(|s| s == "--receipt").unwrap() + 1]);
            fs::write(&receipt,format!("{{\"format\":\"mantle-creation-receipt/v1\",\"selection\":{selection},\"session\":{},\"created_at\":\"time\"}}",session())).unwrap();
            fs::set_permissions(receipt, fs::Permissions::from_mode(0o600)).unwrap();
            println!("synthetic startup text must never enter reports");
        }
        "status" => {
            let body = session();
            let outcome = if body.contains("STOPPED") {
                "recorded"
            } else {
                "observed"
            };
            println!(
                "{{\"format\":\"mantle-session-metadata/v1\",\"selection\":{selection},\"outcome\":\"{outcome}\",\"observed_at\":\"time\",\"sessions\":[{body}]}}"
            );
        }
        "stop" | "restart" | "destroy" | "exec" | "attach" => {
            assert_eq!(
                args[args
                    .iter()
                    .position(|s| s == "--expected-session-id")
                    .unwrap()
                    + 1],
                "owned"
            );
            if mode == "reuse" {
                std::process::exit(1);
            }
            if mode == "flood" {
                loop {
                    std::io::stdout().write_all(&[b'x'; 8192]).unwrap();
                }
            }
            if mode == "timeout" {
                std::thread::sleep(std::time::Duration::from_secs(300));
            }
            let data = fs::read_to_string(&session_path).unwrap();
            let mut fields: Vec<_> = data.split('|').map(str::to_owned).collect();
            match verb.as_str() {
                "stop" => fields[3] = "RETAINED".into(),
                "restart" => {
                    fields[2] = "exec2".into();
                    fields[3] = "RUNNING".into();
                }
                "destroy" => fields[3] = "STOPPED".into(),
                "exec" => {
                    let at = args.iter().position(|s| s == "--").unwrap();
                    if args[at + 1] == "touch" {
                        fs::write(root.join("marker"), "marker").unwrap();
                    } else {
                        assert!(root.join("marker").exists());
                    }
                }
                "attach" => {
                    if mode == "pty-timeout" {
                        let mut descendant = std::process::Command::new("sleep")
                            .arg("300")
                            .spawn()
                            .unwrap();
                        fs::write(root.join("descendant-pid"), descendant.id().to_string())
                            .unwrap();
                        let _ = descendant.wait();
                    }
                    assert!(
                        std::process::Command::new("stty")
                            .args(["raw", "-echo"])
                            .status()
                            .unwrap()
                            .success()
                    );
                    print!("synthetic raw terminal must never enter reports");
                    std::io::stdout().flush().unwrap();
                    let mut previous = 0;
                    loop {
                        let mut byte = [0];
                        std::io::stdin().read_exact(&mut byte).unwrap();
                        if previous == 0x1d && byte[0] == b'd' {
                            break;
                        }
                        previous = byte[0];
                    }
                }
                _ => {}
            }
            fs::write(session_path, fields.join("|")).unwrap();
        }
        _ => panic!("unexpected CLI command"),
    }
}
