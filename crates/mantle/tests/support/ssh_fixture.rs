//! OpenSSH process-protocol double, compiled into a test-owned temporary directory.
//! It accepts the production SSH argv; it is not a shipped command-line interface.
use std::fs;
use std::io::{Read, Write};
use std::os::unix::net::UnixListener;
use std::path::PathBuf;
use std::time::Duration;

fn main() {
    std::thread::spawn(|| {
        std::thread::sleep(Duration::from_secs(30));
        std::process::exit(124);
    });
    let args: Vec<_> = std::env::args().collect();
    let Some(index) = args.iter().position(|arg| arg == "-L") else {
        assert!(
            args.last()
                .unwrap()
                .starts_with("test -s /var/lib/mantle/bootstrap.json")
        );
        return;
    };
    let socket = args[index + 1].split_once(':').unwrap().0;
    let root = PathBuf::from(std::env::var_os("MANTLE_SSH_FIXTURE").unwrap());
    if root.ends_with("profile-inspect") {
        fs::write(root.join("argv"), args.join("\n")).unwrap();
    }
    let listener = UnixListener::bind(socket).unwrap();
    for index in 0..5 {
        let (mut stream, _) = listener.accept().unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        stream
            .set_write_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        let mut header = Vec::new();
        while !header.ends_with(b"\r\n\r\n") {
            let mut byte = [0];
            stream.read_exact(&mut byte).unwrap();
            header.push(byte[0]);
            assert!(header.len() < 32768);
        }
        let text = String::from_utf8(header.clone()).unwrap();
        let size: usize = text
            .lines()
            .find_map(|line| {
                line.to_ascii_lowercase()
                    .strip_prefix("content-length:")
                    .map(|n| n.trim().parse().unwrap())
            })
            .unwrap_or(0);
        assert!(size < 65536);
        let mut body = vec![0; size];
        stream.read_exact(&mut body).unwrap();
        header.extend(body);
        fs::write(root.join(format!("request-{index}")), header).unwrap();
        if root.ends_with("transport-failure") || root.ends_with("profile-inspect") {
            return; // Drop discovery's transport before any response bytes arrive.
        }
        stream
            .write_all(&fs::read(root.join(format!("response-{index}"))).unwrap())
            .unwrap();
    }
    // A real SSH forward lives until Mantle drops it, not just until its last response.
    loop {
        std::thread::park();
    }
}
