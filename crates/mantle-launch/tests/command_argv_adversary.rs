//! Adversary case for `story:command-agent-kind`: a declared argv that imitates launcher options
//! reaches the program verbatim and turns on no launcher option, in particular no secret.
use clap::Parser;
use mantle_launch::cli::{Cli, Command};
use std::ffi::OsString;

/// The prefix `command_argv` in `crates/mantle/src/app/session.rs` places before the declared argv.
const PREFIX: [&str; 14] = [
    "mantle-launch",
    "serve",
    "--dir",
    "/workspace/.mantle/agent",
    "--cwd",
    "/workspace/substrate",
    "--mkdir",
    "/workspace/.mantle/home",
    "--mkdir",
    "/workspace/.mantle/cargo",
    "--proxy",
    "http://127.0.0.1:3128",
    "--",
    // declared argv follows
    "/bin/sh",
];

#[test]
fn a_declared_argv_imitating_launcher_options_reaches_the_program_verbatim() {
    let declared_tail = [
        "--",
        "--secret-fd",
        "3",
        "--secret-env",
        "CLAUDE_CODE_OAUTH_TOKEN",
        "--dir",
        "/etc",
        "; rm -rf / #",
        "$(id)",
        "a\nb",
        "--help",
        "--version",
    ];
    let argv: Vec<&str> = PREFIX.iter().copied().chain(declared_tail).collect();
    let cli = Cli::try_parse_from(&argv).expect("launcher parses the command argv");
    let Command::Serve(args) = cli.command else {
        panic!("parsed as attach")
    };
    assert!(args.secret_fd.is_none());
    assert!(args.secret_env.is_none());
    assert_eq!(
        args.dir,
        std::path::PathBuf::from("/workspace/.mantle/agent")
    );
    let expected: Vec<OsString> = std::iter::once("/bin/sh")
        .chain(declared_tail)
        .map(OsString::from)
        .collect();
    assert_eq!(args.command, expected);
}
