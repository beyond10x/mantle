use std::ffi::OsString;
use std::os::unix::io::RawFd;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

pub const DEFAULT_SCROLLBACK_BYTES: u32 = 256 * 1024;

/// In-sandbox launcher: `serve` keeps the agent on a pseudo-terminal that outlives any terminal
/// attached to it; `attach` relays a terminal to that server through named pipes.
#[derive(Debug, Parser)]
#[command(name = "mantle-launch", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Run the agent on a pseudo-terminal and serve it to one attached terminal at a time.
    Serve(ServeArgs),
    /// Relay this terminal to a running session server.
    Attach(AttachArgs),
}

#[derive(Debug, clap::Args)]
pub struct ServeArgs {
    /// Session directory holding the named pipes and locks.
    #[arg(long, value_name = "DIR", value_parser = absolute_path)]
    pub dir: PathBuf,

    /// Inherited descriptor holding the agent credential.
    #[arg(long, value_name = "FD", requires = "secret_env",
          value_parser = clap::value_parser!(i32).range(3..))]
    pub secret_fd: Option<RawFd>,

    /// Environment variable the credential is exported under, in the agent's environment only.
    #[arg(long, value_name = "NAME", requires = "secret_fd", value_parser = env_name)]
    pub secret_env: Option<String>,

    /// HTTP proxy the agent's tools use, exported as `HTTPS_PROXY` and its variants. Substrate's
    /// API admits no proxy variable, so the launcher sets them inside the sandbox.
    #[arg(long, value_name = "URL", value_parser = proxy_url)]
    pub proxy: Option<String>,

    /// Working directory of the agent.
    #[arg(long, value_name = "DIR", value_parser = absolute_path)]
    pub cwd: PathBuf,

    /// Directory to create before starting (repeatable).
    #[arg(long = "mkdir", value_name = "DIR", value_parser = absolute_path)]
    pub mkdirs: Vec<PathBuf>,

    /// Create/check this private directory without following any path component.
    #[arg(long = "private-dir", value_name = "DIR", value_parser = absolute_path)]
    pub private_dirs: Vec<PathBuf>,

    /// Create/check an owner-only directory and require an observed tmpfs filesystem.
    #[arg(long = "volatile-dir", value_name = "DIR", value_parser = absolute_path)]
    pub volatile_dirs: Vec<PathBuf>,

    /// Permit an absent file or an owned regular, singly-linked 0600 file; never read it.
    #[arg(long = "check-private-file", value_name = "FILE", value_parser = absolute_path)]
    pub check_private_files: Vec<PathBuf>,

    /// Agent output kept for replay to a terminal that attaches later.
    #[arg(long, value_name = "BYTES", default_value_t = DEFAULT_SCROLLBACK_BYTES,
          value_parser = clap::value_parser!(u32).range(1024..=64 * 1024 * 1024))]
    pub scrollback_bytes: u32,

    /// Keep replay in memory only; refuse an existing last-output and never write one.
    #[arg(long)]
    pub volatile_replay: bool,

    /// Agent program and its arguments, after `--`.
    #[arg(last = true, required = true, num_args = 1.., value_name = "PROGRAM")]
    pub command: Vec<OsString>,
}

#[derive(Debug, clap::Args)]
pub struct AttachArgs {
    /// Session directory of the running server.
    #[arg(long, value_name = "DIR", value_parser = absolute_path)]
    pub dir: PathBuf,

    /// Hold replay until the Mantle client acknowledges terminal readiness (10 second bound).
    #[arg(long)]
    pub wait_ready: bool,

    /// Relay plain standard input and output: no raw mode, no window size. For tests.
    #[arg(long, hide = true)]
    pub no_tty: bool,
}

/// `http://host:port` and nothing else: userinfo would put a credential into every child's
/// environment.
fn proxy_url(value: &str) -> Result<String, String> {
    let Some(authority) = value.strip_prefix("http://") else {
        return Err("must start with http://".into());
    };
    let authority = authority.strip_suffix('/').unwrap_or(authority);
    let Some((host, port)) = authority.rsplit_once(':') else {
        return Err("must name a port".into());
    };
    let host_ok = !host.is_empty()
        && host
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-'));
    if !host_ok || port.parse::<u16>().is_err() {
        return Err("must be http://HOST:PORT with no userinfo or path".into());
    }
    Ok(value.to_owned())
}

fn absolute_path(value: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(value);
    if !path.is_absolute() {
        return Err("must be an absolute path".into());
    }
    Ok(path)
}

fn env_name(value: &str) -> Result<String, String> {
    let mut chars = value.chars();
    let valid_start = chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_');
    if !valid_start || !chars.all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err("must match [A-Za-z_][A-Za-z0-9_]*".into());
    }
    Ok(value.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(extra: &[&str]) -> Result<Cli, clap::Error> {
        let mut argv = vec!["mantle-launch"];
        argv.extend_from_slice(extra);
        Cli::try_parse_from(argv)
    }

    fn serve(extra: &[&str]) -> Result<ServeArgs, clap::Error> {
        match parse(extra)?.command {
            Command::Serve(args) => Ok(args),
            Command::Attach(_) => panic!("parsed as attach"),
        }
    }

    const BASE: &[&str] = &[
        "serve",
        "--dir",
        "/workspace/.mantle/agent",
        "--cwd",
        "/workspace/substrate",
    ];

    fn with(prefix: &[&'static str], suffix: &[&'static str]) -> Vec<&'static str> {
        let mut v = BASE.to_vec();
        v.extend_from_slice(prefix);
        v.extend_from_slice(suffix);
        v
    }

    #[test]
    fn full_command_line_parses() {
        let args = serve(&with(
            &[
                "--secret-fd",
                "3",
                "--secret-env",
                "CLAUDE_CODE_OAUTH_TOKEN",
                "--proxy",
                "http://127.0.0.1:3128",
                "--scrollback-bytes",
                "4096",
            ],
            &[
                "--mkdir",
                "/workspace/.mantle/home",
                "--mkdir",
                "/workspace/.mantle/cargo",
                "--",
                "/opt/mantle/bin/claude",
                "--flag",
                "x",
            ],
        ))
        .unwrap();
        assert_eq!(args.secret_fd, Some(3));
        assert_eq!(args.secret_env.as_deref(), Some("CLAUDE_CODE_OAUTH_TOKEN"));
        assert_eq!(args.proxy.as_deref(), Some("http://127.0.0.1:3128"));
        assert_eq!(args.scrollback_bytes, 4096);
        assert_eq!(args.mkdirs.len(), 2);
        assert_eq!(args.dir, PathBuf::from("/workspace/.mantle/agent"));
        assert_eq!(args.command, ["/opt/mantle/bin/claude", "--flag", "x"]);
    }

    #[test]
    fn secret_proxy_and_scrollback_are_optional() {
        let args = serve(&with(&[], &["--", "/bin/sh"])).unwrap();
        assert_eq!(args.secret_fd, None);
        assert_eq!(args.proxy, None);
        assert_eq!(args.scrollback_bytes, DEFAULT_SCROLLBACK_BYTES);
        assert!(args.mkdirs.is_empty());
    }

    #[test]
    fn program_is_required_after_separator() {
        assert!(serve(&with(&[], &[])).is_err());
        assert!(serve(&with(&[], &["--"])).is_err());
        assert!(serve(&with(&[], &["/bin/sh"])).is_err());
    }

    #[test]
    fn dir_and_cwd_are_required() {
        assert!(parse(&["serve", "--cwd", "/w", "--", "/bin/sh"]).is_err());
        assert!(parse(&["serve", "--dir", "/d", "--", "/bin/sh"]).is_err());
        assert!(parse(&["attach"]).is_err());
    }

    #[test]
    fn removed_tmux_flags_are_refused() {
        assert!(serve(&with(&["--tmux-socket", "/w/t.sock"], &["--", "/bin/sh"])).is_err());
        assert!(serve(&with(&["--session", "agent"], &["--", "/bin/sh"])).is_err());
    }

    #[test]
    fn secret_fd_and_env_require_each_other() {
        assert!(serve(&with(&["--secret-fd", "3"], &["--", "/bin/sh"])).is_err());
        assert!(serve(&with(&["--secret-env", "TOKEN"], &["--", "/bin/sh"])).is_err());
    }

    #[test]
    fn standard_descriptors_are_refused() {
        for fd in ["0", "1", "2", "-1"] {
            let argv = with(
                &["--secret-env", "TOKEN", "--secret-fd", fd],
                &["--", "/bin/sh"],
            );
            assert!(serve(&argv).is_err(), "fd {fd} accepted");
        }
    }

    #[test]
    fn scrollback_is_bounded() {
        for bad in ["0", "1023", "67108865", "-1", "x"] {
            let argv = with(&["--scrollback-bytes", bad], &["--", "/bin/sh"]);
            assert!(serve(&argv).is_err(), "{bad} accepted");
        }
    }

    #[test]
    fn attach_parses_and_hides_no_tty() {
        match parse(&["attach", "--dir", "/workspace/.mantle/agent"])
            .unwrap()
            .command
        {
            Command::Attach(args) => assert!(!args.no_tty),
            Command::Serve(_) => panic!("parsed as serve"),
        }
        match parse(&["attach", "--dir", "/d", "--no-tty"])
            .unwrap()
            .command
        {
            Command::Attach(args) => assert!(args.no_tty),
            Command::Serve(_) => panic!("parsed as serve"),
        }
        assert!(parse(&["attach", "--dir", "relative"]).is_err());
    }

    #[test]
    fn env_names_are_validated() {
        assert!(env_name("CLAUDE_CODE_OAUTH_TOKEN").is_ok());
        assert!(env_name("_x1").is_ok());
        for bad in ["", "1A", "A=B", "A B", "A-B", "Ä"] {
            assert!(env_name(bad).is_err(), "{bad:?} accepted");
        }
    }

    #[test]
    fn proxy_urls_carry_no_userinfo_or_path() {
        assert!(proxy_url("http://egress:3128").is_ok());
        assert!(proxy_url("http://egress:3128/").is_ok());
        for bad in [
            "egress:3128",
            "https://egress:3128",
            "http://user:pw@egress:3128",
            "http://egress",
            "http://egress:99999",
            "http://egress:3128/path",
            "http://:3128",
        ] {
            assert!(proxy_url(bad).is_err(), "{bad:?} accepted");
        }
    }

    #[test]
    fn paths_must_be_absolute() {
        assert!(absolute_path("relative/dir").is_err());
        assert!(absolute_path("/workspace").is_ok());
        assert!(serve(&["serve", "--dir", "rel", "--cwd", "/w", "--", "/bin/sh"]).is_err());
    }
}
