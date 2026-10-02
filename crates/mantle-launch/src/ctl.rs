//! The control pipe carries window sizes as `COLS ROWS\n` lines from `attach` to `serve`.

pub const MAX_DIMENSION: u16 = 1000;
const MAX_LINE: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Window {
    pub cols: u16,
    pub rows: u16,
}

impl Window {
    /// The window if both dimensions are within 1..=MAX_DIMENSION.
    pub fn bounded(self) -> Option<Self> {
        let ok = |v: u16| (1..=MAX_DIMENSION).contains(&v);
        (ok(self.cols) && ok(self.rows)).then_some(self)
    }

    pub fn line(self) -> String {
        format!("{} {}\n", self.cols, self.rows)
    }
}

/// Parses one line without its newline: two decimal numbers, columns then rows.
pub fn parse_window(line: &str) -> Option<Window> {
    let mut fields = line.split_ascii_whitespace();
    let cols = fields.next()?.parse().ok()?;
    let rows = fields.next()?.parse().ok()?;
    if fields.next().is_some() {
        return None;
    }
    Window { cols, rows }.bounded()
}

/// Splits the control stream into lines. Malformed and over-long lines are dropped.
#[derive(Debug, Default)]
pub struct WindowLines {
    partial: Vec<u8>,
    discarding: bool,
}

impl WindowLines {
    pub fn feed(&mut self, bytes: &[u8]) -> Vec<Window> {
        let mut windows = Vec::new();
        for &byte in bytes {
            if byte == b'\n' {
                if !self.discarding
                    && let Some(window) = std::str::from_utf8(&self.partial)
                        .ok()
                        .and_then(parse_window)
                {
                    windows.push(window);
                }
                self.partial.clear();
                self.discarding = false;
            } else if self.discarding {
            } else if self.partial.len() >= MAX_LINE {
                self.partial.clear();
                self.discarding = true;
            } else {
                self.partial.push(byte);
            }
        }
        windows
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn w(cols: u16, rows: u16) -> Window {
        Window { cols, rows }
    }

    #[test]
    fn parses_columns_then_rows() {
        assert_eq!(parse_window("80 24"), Some(w(80, 24)));
        assert_eq!(parse_window("  200\t50 \r"), Some(w(200, 50)));
        assert_eq!(parse_window("1 1000"), Some(w(1, 1000)));
    }

    #[test]
    fn refuses_malformed_and_out_of_bounds_lines() {
        for bad in [
            "", "80", "80 24 1", "0 24", "80 0", "1001 24", "80 1001", "-1 24", "80x24", "a b",
            "70000 24",
        ] {
            assert_eq!(parse_window(bad), None, "{bad:?} accepted");
        }
    }

    #[test]
    fn line_round_trips() {
        assert_eq!(w(132, 43).line(), "132 43\n");
        assert_eq!(parse_window(w(132, 43).line().trim_end()), Some(w(132, 43)));
    }

    #[test]
    fn lines_split_across_reads_are_joined() {
        let mut lines = WindowLines::default();
        assert!(lines.feed(b"12").is_empty());
        assert_eq!(lines.feed(b"0 4"), vec![]);
        assert_eq!(lines.feed(b"0\n90 30\nbad\n"), vec![w(120, 40), w(90, 30)]);
    }

    #[test]
    fn over_long_lines_are_dropped_whole() {
        let mut lines = WindowLines::default();
        let mut long = vec![b' '; 100];
        long.extend_from_slice(b"80 24\n");
        assert!(lines.feed(&long).is_empty());
        assert_eq!(lines.feed(b"80 24\n"), vec![w(80, 24)]);
    }

    #[test]
    fn non_utf8_is_ignored() {
        let mut lines = WindowLines::default();
        assert!(lines.feed(b"\xff\xfe\n").is_empty());
        assert_eq!(lines.feed(b"2 2\n"), vec![w(2, 2)]);
    }
}
