use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// Why a resolved address may not be dialled, or `None` when it is public.
pub fn refusal(ip: IpAddr) -> Option<&'static str> {
    match ip {
        IpAddr::V4(v4) => refusal_v4(v4),
        IpAddr::V6(v6) => refusal_v6(v6),
    }
}

fn refusal_v4(ip: Ipv4Addr) -> Option<&'static str> {
    let [a, b, c, _] = ip.octets();
    match (a, b, c) {
        (0, _, _) => Some("unspecified"),
        (127, _, _) => Some("loopback"),
        (10, _, _) | (192, 168, _) => Some("private"),
        (172, 16..=31, _) => Some("private"),
        (169, 254, _) => Some("link-local"),
        (100, 64..=127, _) => Some("cgnat"),
        (192, 0, 0) => Some("reserved"),
        (192, 0, 2) | (198, 51, 100) | (203, 0, 113) => Some("documentation"),
        (198, 18..=19, _) => Some("benchmarking"),
        (224..=239, _, _) => Some("multicast"),
        (240..=255, _, _) => Some("reserved"),
        _ => None,
    }
}

fn refusal_v6(ip: Ipv6Addr) -> Option<&'static str> {
    let s = ip.segments();
    if ip.is_unspecified() {
        return Some("unspecified");
    }
    if ip.is_loopback() {
        return Some("loopback");
    }
    // Forms that embed an IPv4 address are judged by that address, so a mapped
    // 169.254.169.254 cannot slip past the IPv4 rules.
    if let Some(v4) = ip.to_ipv4_mapped() {
        return refusal_v4(v4);
    }
    // SIIT IPv4-translated ::ffff:0:a.b.c.d.
    if s[..6] == [0, 0, 0, 0, 0xffff, 0] {
        return refusal_v4(embedded(s[6], s[7]));
    }
    if s[..6] == [0; 6] {
        return Some("reserved");
    }
    // Well-known NAT64 prefix 64:ff9b::/96.
    if s[..6] == [0x64, 0xff9b, 0, 0, 0, 0] {
        return refusal_v4(embedded(s[6], s[7]));
    }
    // Local-use NAT64 prefix 64:ff9b:1::/48. Judged by the trailing IPv4 address when the
    // address has the /96 shape; any other shape is ambiguous and refused.
    if s[..3] == [0x64, 0xff9b, 1] {
        if s[3..6] == [0, 0, 0] {
            return refusal_v4(embedded(s[6], s[7]));
        }
        return Some("reserved");
    }
    if s[..4] == [0x100, 0, 0, 0] {
        return Some("discard");
    }
    match s[0] {
        x if x & 0xfe00 == 0xfc00 => return Some("private"),
        x if x & 0xffc0 == 0xfe80 => return Some("link-local"),
        x if x & 0xffc0 == 0xfec0 => return Some("private"),
        x if x & 0xff00 == 0xff00 => return Some("multicast"),
        // Everything outside global unicast 2000::/3.
        x if x & 0xe000 != 0x2000 => return Some("reserved"),
        _ => {}
    }
    if s[0] == 0x2002 {
        return refusal_v4(embedded(s[1], s[2]));
    }
    match (s[0], s[1], s[2]) {
        (0x2001, 0, _) => Some("reserved"),
        (0x2001, 0xdb8, _) => Some("documentation"),
        (0x2001, 2, 0) => Some("benchmarking"),
        (x, _, _) if x & 0xfff0 == 0x3ff0 => Some("documentation"),
        _ => None,
    }
}

fn embedded(hi: u16, lo: u16) -> Ipv4Addr {
    Ipv4Addr::from((u32::from(hi) << 16) | u32::from(lo))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(s: &str) -> Option<&'static str> {
        refusal(s.parse().unwrap())
    }

    #[test]
    fn refused_ipv4_ranges() {
        for (ip, why) in [
            ("0.0.0.0", "unspecified"),
            ("127.0.0.1", "loopback"),
            ("127.255.255.254", "loopback"),
            ("10.1.2.3", "private"),
            ("172.16.0.1", "private"),
            ("172.31.255.255", "private"),
            ("192.168.1.1", "private"),
            ("169.254.169.254", "link-local"),
            ("169.254.0.1", "link-local"),
            ("100.64.0.1", "cgnat"),
            ("100.127.255.255", "cgnat"),
            ("224.0.0.1", "multicast"),
            ("239.255.255.255", "multicast"),
            ("255.255.255.255", "reserved"),
            ("192.0.0.192", "reserved"),
            ("192.0.2.1", "documentation"),
            ("198.51.100.1", "documentation"),
            ("203.0.113.1", "documentation"),
            ("198.18.0.1", "benchmarking"),
            ("198.19.255.254", "benchmarking"),
        ] {
            assert_eq!(r(ip), Some(why), "{ip}");
        }
    }

    #[test]
    fn refused_ipv6_ranges() {
        for (ip, why) in [
            ("::", "unspecified"),
            ("::1", "loopback"),
            ("fc00::1", "private"),
            ("fd12:3456::1", "private"),
            ("fe80::1", "link-local"),
            ("febf::1", "link-local"),
            ("ff02::1", "multicast"),
            ("::ffff:169.254.169.254", "link-local"),
            ("::ffff:127.0.0.1", "loopback"),
            ("::127.0.0.1", "reserved"),
            ("64:ff9b::a9fe:a9fe", "link-local"),
            ("2002:a9fe:a9fe::1", "link-local"),
            ("2001:0:4136:e378::1", "reserved"),
            ("::ffff:0:a9fe:a9fe", "link-local"),
            ("64:ff9b:1::a9fe:a9fe", "link-local"),
            ("64:ff9b:1:1::8c52:7003", "reserved"),
            ("100::1", "discard"),
            ("2001:db8::1", "documentation"),
            ("2001:2::1", "benchmarking"),
            ("3fff::1", "documentation"),
            ("4000::1", "reserved"),
            ("e000::1", "reserved"),
        ] {
            assert_eq!(r(ip), Some(why), "{ip}");
        }
    }

    #[test]
    fn public_addresses_are_permitted() {
        for ip in [
            "140.82.112.3",
            "1.1.1.1",
            "172.32.0.1",
            "100.128.0.1",
            "172.15.255.255",
            "2606:4700::1111",
            "::ffff:140.82.112.3",
            "64:ff9b::8c52:7003",
            "64:ff9b:1::8c52:7003",
            "::ffff:0:8c52:7003",
            "2001:2:1::1",
            "198.17.255.255",
            "198.20.0.1",
            "192.0.1.1",
        ] {
            assert_eq!(r(ip), None, "{ip}");
        }
    }
}
