use std::io::Write;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// One connection's record. Request headers are deliberately not part of it: they can
/// carry proxy credentials.
pub struct Line<'a> {
    pub dest: &'a str,
    pub outcome: &'a str,
    pub up: u64,
    pub down: u64,
    pub elapsed: Duration,
}

pub fn emit(line: &Line<'_>) {
    let mut err = std::io::stderr().lock();
    let _ = writeln!(
        err,
        "{} dest={} outcome={} up={} down={} ms={}",
        timestamp(SystemTime::now()),
        line.dest,
        line.outcome,
        line.up,
        line.down,
        line.elapsed.as_millis()
    );
}

pub fn notice(msg: &str) {
    let mut err = std::io::stderr().lock();
    let _ = writeln!(err, "{} {msg}", timestamp(SystemTime::now()));
}

/// RFC 3339 UTC with milliseconds, without pulling in a date crate.
pub fn timestamp(t: SystemTime) -> String {
    let d = t.duration_since(UNIX_EPOCH).unwrap_or_default();
    let secs = d.as_secs();
    let days = i64::try_from(secs / 86_400).unwrap_or(0);
    let rem = secs % 86_400;
    let (y, m, day) = civil_from_days(days);
    format!(
        "{y:04}-{m:02}-{day:02}T{:02}:{:02}:{:02}.{:03}Z",
        rem / 3600,
        (rem / 60) % 60,
        rem % 60,
        d.subsec_millis()
    )
}

// Howard Hinnant's days-to-civil algorithm.
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    (
        y,
        u32::try_from(m).unwrap_or(1),
        u32::try_from(d).unwrap_or(1),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_known_instants() {
        assert_eq!(timestamp(UNIX_EPOCH), "1970-01-01T00:00:00.000Z");
        let t = UNIX_EPOCH + Duration::from_millis(1_790_942_400_123);
        assert_eq!(timestamp(t), "2026-10-02T12:00:00.123Z");
        let leap = UNIX_EPOCH + Duration::from_secs(951_782_400);
        assert_eq!(timestamp(leap), "2000-02-29T00:00:00.000Z");
    }
}
