use std::time::Duration;

use tokio::io::{AsyncRead, AsyncReadExt};

use crate::allow::{Destination, TargetError, parse_authority};

pub const MAX_HEAD: usize = 8 * 1024;
const CHUNK: usize = 2048;

#[derive(Debug, PartialEq, Eq)]
pub enum ReadError {
    Closed,
    Timeout,
    TooLarge,
    Io,
}

/// A request head and whatever the client already sent past it (a client may send its
/// TLS hello before seeing the 200), which must be forwarded upstream, not dropped.
#[derive(Debug)]
pub struct RawHead {
    pub head: Vec<u8>,
    pub rest: Vec<u8>,
}

pub async fn read_head<R: AsyncRead + Unpin>(
    reader: &mut R,
    limit: Duration,
) -> Result<RawHead, ReadError> {
    match tokio::time::timeout(limit, read_head_inner(reader)).await {
        Ok(r) => r,
        Err(_) => Err(ReadError::Timeout),
    }
}

async fn read_head_inner<R: AsyncRead + Unpin>(reader: &mut R) -> Result<RawHead, ReadError> {
    let mut buf = Vec::with_capacity(1024);
    let mut chunk = [0u8; CHUNK];
    loop {
        let room = MAX_HEAD - buf.len();
        if room == 0 {
            return Err(ReadError::TooLarge);
        }
        let n = reader
            .read(&mut chunk[..room.min(CHUNK)])
            .await
            .map_err(|_| ReadError::Io)?;
        if n == 0 {
            return Err(ReadError::Closed);
        }
        let from = buf.len().saturating_sub(3);
        buf.extend_from_slice(&chunk[..n]);
        if let Some(pos) = buf[from..].windows(4).position(|w| w == b"\r\n\r\n") {
            let end = from + pos + 4;
            let rest = buf.split_off(end);
            return Ok(RawHead { head: buf, rest });
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum HeadError {
    Malformed(&'static str),
    /// Carries the method token, which is validated as uppercase ASCII letters.
    Method(String),
    Target(TargetError),
}

/// Parses a complete request head (ending in an empty line) and returns the CONNECT
/// destination. Header lines are checked for shape and then ignored.
pub fn parse_head(head: &[u8]) -> Result<Destination, HeadError> {
    if !head.is_ascii() {
        return Err(HeadError::Malformed("non-ASCII request head"));
    }
    let text =
        std::str::from_utf8(head).map_err(|_| HeadError::Malformed("non-ASCII request head"))?;
    let body = text
        .strip_suffix("\r\n\r\n")
        .ok_or(HeadError::Malformed("unterminated request head"))?;
    let mut lines = body.split("\r\n");
    let request_line = lines.next().unwrap_or_default();
    if body.bytes().any(|b| b == b'\0') || body.split("\r\n").any(|l| l.contains(['\r', '\n'])) {
        return Err(HeadError::Malformed("bare CR or LF in request head"));
    }

    let mut parts = request_line.split(' ');
    let (Some(method), Some(target), Some(version), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return Err(HeadError::Malformed(
            "request line is not METHOD TARGET VERSION",
        ));
    };
    if method.is_empty() || method.len() > 16 || !method.bytes().all(|b| b.is_ascii_uppercase()) {
        return Err(HeadError::Malformed("invalid method"));
    }
    if method != "CONNECT" {
        return Err(HeadError::Method(method.to_owned()));
    }
    if version != "HTTP/1.1" {
        return Err(HeadError::Malformed("only HTTP/1.1 is supported"));
    }
    for line in lines {
        let valid = line.split_once(':').is_some_and(|(name, _)| {
            !name.is_empty() && name.bytes().all(|b| b.is_ascii_graphic())
        });
        if !valid {
            return Err(HeadError::Malformed("invalid header line"));
        }
    }
    parse_authority(target).map_err(HeadError::Target)
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIMIT: Duration = Duration::from_secs(5);

    #[test]
    fn valid_connect() {
        let head = b"CONNECT github.com:443 HTTP/1.1\r\nHost: github.com:443\r\nProxy-Authorization: Basic eA==\r\n\r\n";
        let d = parse_head(head).unwrap();
        assert_eq!(d.to_string(), "github.com:443");
    }

    #[test]
    fn connect_without_headers() {
        let d = parse_head(b"CONNECT crates.io:443 HTTP/1.1\r\n\r\n").unwrap();
        assert_eq!(d.host, "crates.io");
        assert_eq!(d.port, 443);
    }

    #[test]
    fn wrong_method() {
        assert_eq!(
            parse_head(b"GET http://github.com/ HTTP/1.1\r\nHost: github.com\r\n\r\n"),
            Err(HeadError::Method("GET".into()))
        );
        assert_eq!(
            parse_head(b"POST github.com:443 HTTP/1.1\r\n\r\n"),
            Err(HeadError::Method("POST".into()))
        );
    }

    #[test]
    fn malformed_heads() {
        let cases: &[&[u8]] = &[
            b"CONNECT github.com:443\r\n\r\n",
            b"CONNECT  github.com:443 HTTP/1.1\r\n\r\n",
            b"CONNECT github.com:443 HTTP/1.1 extra\r\n\r\n",
            b"CONNECT github.com:443 HTTP/1.0\r\n\r\n",
            b"CONNECT github.com:443 HTTP/2\r\n\r\n",
            b"connect github.com:443 HTTP/1.1\r\n\r\n",
            b"CONNECT github.com:443 HTTP/1.1\r\nno colon\r\n\r\n",
            b"CONNECT github.com:443 HTTP/1.1\r\nBad Name: x\r\n\r\n",
            b"CONNECT github.com:443 HTTP/1.1\r\nHost: a\rb\r\n\r\n",
            b"CONNECT github.com:443 HTTP/1.1\nHost: x\r\n\r\n",
            b"CONNECT github.com:443 HTTP/1.1\r\nHost: \xff\r\n\r\n",
            b"\r\n\r\n",
            b"CONNECT github.com:443 HTTP/1.1\r\n",
        ];
        for c in cases {
            assert!(
                matches!(parse_head(c), Err(HeadError::Malformed(_))),
                "{}",
                String::from_utf8_lossy(c)
            );
        }
        assert!(matches!(
            parse_head(b"CONNECT github.com HTTP/1.1\r\n\r\n"),
            Err(HeadError::Target(TargetError::Malformed(_)))
        ));
        assert!(matches!(
            parse_head(b"CONNECT 10.0.0.1:443 HTTP/1.1\r\n\r\n"),
            Err(HeadError::Target(TargetError::IpLiteral(_)))
        ));
    }

    #[tokio::test]
    async fn read_head_keeps_bytes_past_the_head() {
        let mut input: &[u8] = b"CONNECT github.com:443 HTTP/1.1\r\n\r\n\x16\x03\x01hello";
        let raw = read_head(&mut input, LIMIT).await.unwrap();
        assert!(raw.head.ends_with(b"\r\n\r\n"));
        assert_eq!(raw.rest, b"\x16\x03\x01hello");
    }

    /// Yields exactly one queued chunk per read, so a test controls where reads split.
    struct Chunks(std::collections::VecDeque<&'static [u8]>);

    impl AsyncRead for Chunks {
        fn poll_read(
            mut self: std::pin::Pin<&mut Self>,
            _: &mut std::task::Context<'_>,
            buf: &mut tokio::io::ReadBuf<'_>,
        ) -> std::task::Poll<std::io::Result<()>> {
            if let Some(chunk) = self.0.pop_front() {
                assert!(chunk.len() <= buf.remaining(), "chunk larger than read");
                buf.put_slice(chunk);
            }
            std::task::Poll::Ready(Ok(()))
        }
    }

    #[tokio::test]
    async fn read_head_terminator_split_across_reads() {
        let splits: &[&[&'static [u8]]] = &[
            &[b"CONNECT github.com:443 HTTP/1.1\r\n\r", b"\nTLS"],
            &[b"CONNECT github.com:443 HTTP/1.1\r\n", b"\r\nTLS"],
            &[b"CONNECT github.com:443 HTTP/1.1\r", b"\n\r\nTLS"],
            &[
                b"CONNECT github.com:443 HTTP/1.1",
                b"\r",
                b"\n",
                b"\r",
                b"\nTLS",
            ],
        ];
        for chunks in splits {
            let mut input = Chunks(chunks.iter().copied().collect());
            let raw = read_head(&mut input, LIMIT).await.unwrap();
            assert_eq!(raw.head, b"CONNECT github.com:443 HTTP/1.1\r\n\r\n");
            assert_eq!(raw.rest, b"TLS");
            assert_eq!(parse_head(&raw.head).unwrap().to_string(), "github.com:443");
        }
    }

    #[tokio::test]
    async fn read_head_oversize() {
        let mut big = b"CONNECT github.com:443 HTTP/1.1\r\nX: ".to_vec();
        big.extend(std::iter::repeat_n(b'a', MAX_HEAD));
        big.extend_from_slice(b"\r\n\r\n");
        let mut input: &[u8] = &big;
        assert_eq!(
            read_head(&mut input, LIMIT).await.unwrap_err(),
            ReadError::TooLarge
        );
    }

    #[tokio::test]
    async fn read_head_exactly_at_cap() {
        let prefix = b"CONNECT github.com:443 HTTP/1.1\r\nX: ";
        let mut head = prefix.to_vec();
        head.extend(std::iter::repeat_n(b'a', MAX_HEAD - prefix.len() - 4));
        head.extend_from_slice(b"\r\n\r\n");
        assert_eq!(head.len(), MAX_HEAD);
        let mut input: &[u8] = &head;
        assert!(read_head(&mut input, LIMIT).await.is_ok());
    }

    #[tokio::test]
    async fn read_head_closed_early() {
        let mut input: &[u8] = b"CONNECT github.com:443 HTTP/1.1\r\n";
        assert_eq!(
            read_head(&mut input, LIMIT).await.unwrap_err(),
            ReadError::Closed
        );
    }

    #[tokio::test]
    async fn read_head_slow_times_out() {
        let (mut client, mut server) = tokio::io::duplex(64);
        tokio::io::AsyncWriteExt::write_all(&mut client, b"CONNECT gith")
            .await
            .unwrap();
        assert_eq!(
            read_head(&mut server, Duration::from_millis(50))
                .await
                .unwrap_err(),
            ReadError::Timeout
        );
        drop(client);
    }
}
