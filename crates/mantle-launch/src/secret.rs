use std::fs::File;
use std::io::Read;
use std::os::fd::FromRawFd;
use std::os::unix::io::RawFd;

use anyhow::{Context, Result, bail};

pub const MAX_SECRET_BYTES: usize = 64 * 1024;

/// Reads the credential from an inherited descriptor and closes it.
/// Errors name the descriptor and variable, never the value.
pub fn read_from_fd(fd: RawFd, env_name: &str) -> Result<String> {
    let file = take_fd(fd)?;
    read_secret(file).with_context(|| format!("secret descriptor {fd} for {env_name}"))
}

#[allow(unsafe_code)]
fn take_fd(fd: RawFd) -> Result<File> {
    // SAFETY: F_GETFD only inspects the descriptor table; it is a liveness probe so that an
    // unopened number is reported instead of being adopted by a File.
    if unsafe { libc::fcntl(fd, libc::F_GETFD) } == -1 {
        bail!("secret descriptor {fd} is not open");
    }
    // SAFETY: the descriptor is open, was handed to this process solely to carry the secret,
    // and nothing else in the process holds it; the File becomes its single owner and closes it.
    Ok(unsafe { File::from_raw_fd(fd) })
}

pub fn read_secret(reader: impl Read) -> Result<String> {
    let mut buf = Vec::new();
    reader
        .take(MAX_SECRET_BYTES as u64 + 1)
        .read_to_end(&mut buf)
        .context("could not be read")?;
    if buf.len() > MAX_SECRET_BYTES {
        bail!("is larger than {MAX_SECRET_BYTES} bytes");
    }
    let text = String::from_utf8(buf).map_err(|_| anyhow::anyhow!("is not UTF-8"))?;
    let value = text.trim_end();
    if value.is_empty() {
        bail!("is empty");
    }
    if value.contains('\0') {
        bail!("contains a NUL byte");
    }
    Ok(value.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::os::fd::IntoRawFd;

    fn via_pipe(content: &[u8]) -> Result<String> {
        let (reader, mut writer) = std::io::pipe().unwrap();
        let payload = content.to_vec();
        let feeder = std::thread::spawn(move || {
            let _ = writer.write_all(&payload);
        });
        let result = read_from_fd(reader.into_raw_fd(), "TOKEN");
        feeder.join().unwrap();
        result
    }

    #[test]
    fn trailing_newline_is_trimmed() {
        assert_eq!(via_pipe(b"tok-123\n").unwrap(), "tok-123");
        assert_eq!(via_pipe(b"tok-123\r\n").unwrap(), "tok-123");
        assert_eq!(via_pipe(b"tok-123").unwrap(), "tok-123");
    }

    #[test]
    fn empty_value_is_refused() {
        for content in [&b""[..], b"\n", b"  \n\t"] {
            let err = via_pipe(content).unwrap_err();
            let msg = format!("{err:#}");
            assert!(msg.contains("is empty"), "{msg}");
            assert!(msg.contains("TOKEN"), "{msg}");
        }
    }

    #[test]
    fn oversized_value_is_refused() {
        let big = vec![b'a'; MAX_SECRET_BYTES + 1];
        let msg = format!("{:#}", via_pipe(&big).unwrap_err());
        assert!(msg.contains("larger than"), "{msg}");
        assert!(!msg.contains("aaaa"), "{msg}");
        assert_eq!(
            via_pipe(&big[..MAX_SECRET_BYTES]).unwrap().len(),
            MAX_SECRET_BYTES
        );
    }

    #[test]
    fn errors_do_not_echo_the_value() {
        let msg = format!("{:#}", via_pipe(b"zq-7\0zq-7").unwrap_err());
        assert!(msg.contains("NUL"), "{msg}");
        assert!(!msg.contains("zq-7"), "{msg}");
        let msg = format!("{:#}", via_pipe(b"tok-9\xff").unwrap_err());
        assert!(msg.contains("UTF-8") && !msg.contains("tok-9"), "{msg}");
    }

    #[test]
    fn unopened_descriptor_is_reported() {
        let msg = format!("{:#}", read_from_fd(i32::MAX, "TOKEN").unwrap_err());
        assert!(msg.contains("descriptor 2147483647 is not open"), "{msg}");
    }
}
