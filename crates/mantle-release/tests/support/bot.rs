use std::{env, fs, io::Write, path::Path, process::Command};
fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    assert_eq!(args.first().map(String::as_str), Some("gh"));
    let command = &args[args.iter().position(|a| a == "--").unwrap() + 1..];
    let trace = env::var("FIXTURE_TRACE").unwrap();
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&trace)
        .unwrap();
    writeln!(file, "{}", command.join(" ")).unwrap();
    let result = format!("{trace}.release");
    let mode = env::var("FIXTURE_MODE").unwrap();
    let head = env::var("FIXTURE_HEAD").unwrap();
    if command[0] == "api" {
        let path = command.last().unwrap();
        if path.contains("/git/ref/tags/") {
            let sha = if mode == "wrong-remote"
                || (mode == "changed-after-create" && Path::new(&result).exists())
            {
                "0000000000000000000000000000000000000000"
            } else {
                &head
            };
            let kind = if mode == "annotated" { "tag" } else { "commit" };
            println!("HTTP/2.0 200 OK\n\n{{\"object\":{{\"type\":\"{kind}\",\"sha\":\"{sha}\"}}}}");
        } else if path.contains("/git/tags/") {
            assert_eq!(mode, "annotated");
            println!(
                "HTTP/2.0 200 OK\n\n{{\"object\":{{\"type\":\"commit\",\"sha\":\"{head}\"}}}}"
            );
        } else if path.contains("/releases/tags/") {
            if mode == "existing" {
                println!("HTTP/2.0 200 OK\n\n{{\"tag_name\":\"0.1.4\"}}");
            } else if mode == "network" {
                println!("HTTP/2.0 503 Unavailable\n\n{{}}");
                std::process::exit(1);
            } else if let Ok(response) = fs::read_to_string(&result) {
                println!("HTTP/2.0 200 OK\n\n{response}");
            } else {
                println!("HTTP/2.0 404 Not Found\n\n{{\"message\":\"Not Found\"}}");
                std::process::exit(1);
            }
        } else {
            panic!("unexpected API path");
        }
    } else if command[0] == "release" && command[1] == "create" {
        if mode == "create-failure" {
            eprintln!("fixture upload failure");
            std::process::exit(42);
        }
        assert!(matches!(
            mode.as_str(),
            "success" | "annotated" | "bad-digest" | "changed-after-create"
        ));
        let start = command
            .iter()
            .position(|arg| arg == "--notes-file")
            .unwrap()
            + 2;
        let mut assets = Vec::new();
        for path in &command[start..] {
            let output = Command::new("sha256sum").arg(path).output().unwrap();
            assert!(output.status.success());
            let sum = String::from_utf8(output.stdout).unwrap();
            let digest = if mode == "bad-digest" {
                "unverified"
            } else {
                sum.split_whitespace().next().unwrap()
            };
            let name = Path::new(path).file_name().unwrap().to_str().unwrap();
            assets.push(format!(
                "{{\"name\":\"{name}\",\"digest\":\"sha256:{digest}\"}}"
            ));
        }
        fs::write(
            result,
            format!(
                "{{\"tag_name\":\"0.1.4\",\"assets\":[{}]}}",
                assets.join(",")
            ),
        )
        .unwrap();
    } else {
        panic!("unexpected bot command");
    }
}
