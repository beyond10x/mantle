use std::fmt;
use std::net::{Ipv4Addr, Ipv6Addr};

/// The allowlist used when no `--allow` is given.
pub const DEFAULT_ALLOW: &[&str] = &[
    "api.anthropic.com:443",
    "platform.claude.com:443",
    "github.com:443",
    "codeload.github.com:443",
    "objects.githubusercontent.com:443",
    "index.crates.io:443",
    "static.crates.io:443",
    "crates.io:443",
    "auth.openai.com:443",
    "chatgpt.com:443",
];

/// A validated `host:port` with the host lower-cased and its trailing dot removed.
/// The host only ever contains `[a-z0-9._-]`, so it is safe to log.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Destination {
    pub host: String,
    pub port: u16,
}

impl Destination {
    /// The name handed to the resolver: fully qualified, so a resolver search list
    /// (`search corp.example`) cannot turn an allowed `github.com` into another name.
    pub fn resolver_name(&self) -> String {
        format!("{}.", self.host)
    }
}

impl fmt::Display for Destination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetError {
    Malformed(&'static str),
    /// Carries the literal in canonical form, safe to log.
    IpLiteral(String),
}

/// Parses an authority-form request target (`host:port`).
pub fn parse_authority(target: &str) -> Result<Destination, TargetError> {
    if let Some(rest) = target.strip_prefix('[') {
        let (inner, _) = rest
            .split_once(']')
            .ok_or(TargetError::Malformed("unterminated IPv6 literal"))?;
        return match inner.parse::<Ipv6Addr>() {
            Ok(ip) => Err(TargetError::IpLiteral(ip.to_string())),
            Err(_) => Err(TargetError::Malformed("invalid IPv6 literal")),
        };
    }
    let (host, port) = target
        .rsplit_once(':')
        .ok_or(TargetError::Malformed("target has no port"))?;
    if host.contains(':') {
        return match host.parse::<Ipv6Addr>() {
            Ok(ip) => Err(TargetError::IpLiteral(ip.to_string())),
            Err(_) => Err(TargetError::Malformed("invalid host")),
        };
    }
    let port = parse_port(port)?;
    let host = normalize_host(host)?;
    if is_ip_literal(&host) {
        return Err(TargetError::IpLiteral(host));
    }
    Ok(Destination { host, port })
}

fn parse_port(port: &str) -> Result<u16, TargetError> {
    if port.is_empty() || port.len() > 5 || !port.bytes().all(|b| b.is_ascii_digit()) {
        return Err(TargetError::Malformed("invalid port"));
    }
    match port.parse::<u16>() {
        Ok(0) | Err(_) => Err(TargetError::Malformed("invalid port")),
        Ok(p) => Ok(p),
    }
}

fn normalize_host(host: &str) -> Result<String, TargetError> {
    let host = host.strip_suffix('.').unwrap_or(host).to_ascii_lowercase();
    if host.is_empty() || host.len() > 253 {
        return Err(TargetError::Malformed("invalid host"));
    }
    for label in host.split('.') {
        let valid = !label.is_empty()
            && label.len() <= 63
            && label
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_');
        if !valid {
            return Err(TargetError::Malformed("invalid host"));
        }
    }
    Ok(host)
}

/// The resolver accepts shorthand IPv4 forms (`127.1`, `2130706433`, `0x7f.1`); a DNS
/// top-level label is never numeric, so a numeric or hex last label marks a literal.
fn is_ip_literal(host: &str) -> bool {
    if host.parse::<Ipv4Addr>().is_ok() {
        return true;
    }
    let last = host.rsplit('.').next().unwrap_or(host);
    last.bytes().all(|b| b.is_ascii_digit()) || last.starts_with("0x")
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Allowlist {
    entries: Vec<Destination>,
}

impl Allowlist {
    pub fn new(entries: Vec<Destination>) -> Self {
        Self { entries }
    }

    pub fn from_specs<S: AsRef<str>>(specs: &[S]) -> Result<Self, String> {
        specs
            .iter()
            .map(|s| parse_allow_entry(s.as_ref()))
            .collect::<Result<Vec<_>, _>>()
            .map(Self::new)
    }

    pub fn default_list() -> Self {
        Self::from_specs(DEFAULT_ALLOW).expect("default allowlist is valid")
    }

    pub fn permits(&self, dest: &Destination) -> bool {
        self.entries.iter().any(|e| e == dest)
    }

    pub fn entries(&self) -> &[Destination] {
        &self.entries
    }
}

/// Parses one `--allow` value. IP literals are rejected because requests for them are
/// always refused, so such an entry could never match.
pub fn parse_allow_entry(spec: &str) -> Result<Destination, String> {
    match parse_authority(spec) {
        Ok(d) => Ok(d),
        Err(TargetError::IpLiteral(_)) => Err(format!(
            "{spec}: IP-literal destinations cannot be allowed; use a host name"
        )),
        Err(TargetError::Malformed(why)) => Err(format!("{spec}: {why}; expected host:port")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dest(host: &str, port: u16) -> Destination {
        Destination {
            host: host.into(),
            port,
        }
    }

    #[test]
    fn default_list_has_ten_exact_hosts_on_443() {
        let list = Allowlist::default_list();
        assert_eq!(list.entries().len(), 10);
        assert!(list.entries().iter().all(|d| d.port == 443));
        assert_eq!(
            list.entries()
                .iter()
                .map(|d| d.host.as_str())
                .collect::<Vec<_>>(),
            [
                "api.anthropic.com",
                "platform.claude.com",
                "github.com",
                "codeload.github.com",
                "objects.githubusercontent.com",
                "index.crates.io",
                "static.crates.io",
                "crates.io",
                "auth.openai.com",
                "chatgpt.com",
            ]
        );
    }

    #[test]
    fn match_is_case_insensitive() {
        let list = Allowlist::default_list();
        let d = parse_authority("GitHub.COM:443").unwrap();
        assert_eq!(d, dest("github.com", 443));
        assert!(list.permits(&d));
    }

    #[test]
    fn trailing_dot_is_stripped() {
        let list = Allowlist::default_list();
        let d = parse_authority("index.crates.io.:443").unwrap();
        assert!(list.permits(&d));
        assert_eq!(
            parse_authority("crates.io..:443"),
            Err(TargetError::Malformed("invalid host"))
        );
    }

    #[test]
    fn port_must_match_exactly() {
        let list = Allowlist::default_list();
        assert!(!list.permits(&parse_authority("github.com:80").unwrap()));
        assert!(!list.permits(&parse_authority("github.com:4443").unwrap()));
        assert!(matches!(
            parse_authority("github.com:0443"),
            Ok(d) if d.port == 443
        ));
    }

    #[test]
    fn match_is_exact_not_suffix() {
        let list = Allowlist::default_list();
        for t in [
            "evil.github.com:443",
            "github.com.evil.example:443",
            "xgithub.com:443",
            "example.com:443",
        ] {
            assert!(!list.permits(&parse_authority(t).unwrap()), "{t}");
        }
    }

    #[test]
    fn ip_literals_are_refused() {
        for (t, lit) in [
            ("127.0.0.1:443", "127.0.0.1"),
            ("169.254.169.254:80", "169.254.169.254"),
            ("[::1]:443", "::1"),
            ("[fe80::1]:443", "fe80::1"),
            ("127.1:443", "127.1"),
            ("2130706433:443", "2130706433"),
            ("0x7f.1:443", "0x7f.1"),
        ] {
            assert_eq!(
                parse_authority(t),
                Err(TargetError::IpLiteral(lit.into())),
                "{t}"
            );
        }
    }

    #[test]
    fn malformed_targets() {
        for t in [
            "github.com",
            "github.com:",
            "github.com:0",
            "github.com:65536",
            "github.com:+443",
            ":443",
            "git hub.com:443",
            "github.com/x:443",
            "user@github.com:443",
            "[::1:443",
            "[nope]:443",
        ] {
            assert!(
                matches!(parse_authority(t), Err(TargetError::Malformed(_))),
                "{t}"
            );
        }
    }

    #[test]
    fn resolver_name_is_fully_qualified() {
        for t in ["github.com:443", "GitHub.com.:443", "index.crates.io:443"] {
            let name = parse_authority(t).unwrap().resolver_name();
            assert!(name.ends_with('.'), "{t} -> {name}");
            assert!(!name.ends_with(".."), "{t} -> {name}");
        }
        assert_eq!(dest("github.com", 443).resolver_name(), "github.com.");
    }

    #[test]
    fn allow_entry_rejects_ip_literal() {
        assert!(parse_allow_entry("10.0.0.1:443").is_err());
        assert!(parse_allow_entry("example.com").is_err());
        assert_eq!(
            parse_allow_entry("Example.com.:8443"),
            Ok(dest("example.com", 8443))
        );
    }
}
