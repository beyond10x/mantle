//! OpenSSH command-protocol double. No provider, service or real worker is contacted.
use std::{
    env, fs,
    io::{Read, Write},
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    process::{Command, Stdio},
};
fn words(text: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut token = String::new();
    let mut quoted = false;
    let mut escaped = false;
    let mut started = false;
    for c in text.chars() {
        if escaped {
            token.push(c);
            escaped = false;
            started = true;
        } else if c == '\\' && !quoted {
            escaped = true;
        } else if c == '\'' {
            quoted = !quoted;
            started = true;
        } else if c.is_whitespace() && !quoted {
            if started {
                result.push(std::mem::take(&mut token));
                started = false;
            }
        } else {
            token.push(c);
            started = true;
        }
    }
    assert!(!quoted && !escaped);
    if started {
        result.push(token);
    }
    result
}
fn main() {
    let root = PathBuf::from(env::var_os("UPGRADE_FIXTURE_HOST").unwrap());
    let trace = env::var_os("UPGRADE_FIXTURE_TRACE").unwrap();
    let command = env::args().last().unwrap();
    let mut log = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(trace)
        .unwrap();
    writeln!(log, "{command}").unwrap();
    let mut args = words(&command);
    if args.get(2).is_some_and(|s| s == "tee") {
        assert_eq!(&args[..4], ["sudo", "-n", "tee", "--"]);
        assert_eq!(args[5], ">/dev/null");
        let mut bytes = Vec::new();
        std::io::stdin().read_to_end(&mut bytes).unwrap();
        fs::write(root.join(args[4].trim_start_matches('/')), bytes).unwrap();
        return;
    }
    assert_eq!(&args[..4], ["sudo", "-n", "env", "LC_ALL=C"]);
    args.drain(..4);
    if args[0] == "mktemp" {
        assert_eq!(&args[1..], ["-d", "/var/tmp/mantle-delivery.XXXXXXXXXXXX"]);
        let path = root.join("var/tmp/mantle-delivery.123456abcdef");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::create_dir(&path).unwrap();
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
        println!("/var/tmp/mantle-delivery.123456abcdef");
        return;
    }
    if args[0].starts_with("/var/tmp/mantle-delivery.") {
        if args[1] == "--version" {
            println!("mantle-worker 0.1.4");
            return;
        }
        assert_eq!(&args[1..3], ["upgrade-apply", "--manifest"]);
        let report = root.join("apply-report.json");
        let output = Command::new(env::var_os("UPGRADE_FIXTURE_TEST_EXE").unwrap())
            .args(["--exact", "upgrade_worker_transaction_child", "--nocapture"])
            .env(
                "UPGRADE_FIXTURE_APPLY_MANIFEST",
                root.join(args[3].trim_start_matches('/')),
            )
            .env("UPGRADE_FIXTURE_APPLY_REPORT", &report)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        std::io::stdout()
            .write_all(&fs::read(report).unwrap())
            .unwrap();
        return;
    }
    if args[0] == "rm" {
        assert_eq!(&args[..3], ["rm", "-r", "--"]);
        assert_eq!(args[3], "/var/tmp/mantle-delivery.123456abcdef");
        fs::remove_dir_all(root.join(args[3].trim_start_matches('/'))).unwrap();
        return;
    }
    if args[0] == "systemctl" {
        if args.iter().any(|a| a == "show") {
            let unit = args.last().unwrap();
            print!(
                "{}",
                fs::read_to_string(root.join(format!("show-{unit}"))).unwrap()
            );
        } else if args.iter().any(|a| a == "list-jobs") {
            print!("{}", fs::read_to_string(root.join("jobs")).unwrap());
        } else {
            panic!("mutating or unsupported service operation");
        }
        return;
    }
    if args[0] == "/usr/local/bin/substrate-daemon" {
        println!("substrate-daemon 0.7.10");
        return;
    }
    if args[0].starts_with("/opt/mantle/bin/") {
        println!("{} 0.1.3", args[0].rsplit('/').next().unwrap());
        return;
    }
    assert!(
        ["stat", "head", "find", "readlink", "sha256sum", "chmod"].contains(&args[0].as_str()),
        "write attempted by check: {command}"
    );
    let name = args.remove(0);
    for arg in &mut args {
        if arg.starts_with('/') {
            *arg = root
                .join(arg.trim_start_matches('/'))
                .to_string_lossy()
                .into_owned();
        }
    }
    let output = Command::new(&name)
        .args(&args)
        .env("LC_ALL", "C")
        .stdin(Stdio::null())
        .output()
        .unwrap();
    let mut bytes = output.stdout;
    if name == "stat" && output.status.success() {
        let text = String::from_utf8(bytes).unwrap();
        let mut fields: Vec<_> = text.split('|').map(str::to_owned).collect();
        fields[1] = "0".into();
        fields[2] = "0".into();
        bytes = fields.join("|").into_bytes();
    }
    std::io::stdout().write_all(&bytes).unwrap();
    std::io::stderr().write_all(&output.stderr).unwrap();
    std::process::exit(output.status.code().unwrap_or(1));
}
