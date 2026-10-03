//! Test-owned OpenSSH protocol double. The shipped CLI still parses real SDK HTTP replies.
use std::{
    fs,
    io::{Read, Write},
    os::unix::net::UnixListener,
    path::PathBuf,
    time::Duration,
};
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
    let root = PathBuf::from(std::env::var_os("MANTLE_LIFECYCLE_FIXTURE").unwrap());
    let socket = args[index + 1].split_once(':').unwrap().0;
    let listener = UnixListener::bind(socket).unwrap();
    for index in 0..32 {
        let (mut stream, _) = listener.accept().unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
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
        // Test-owned response barrier for concurrent, separate CLI callers.
        if root.join(format!("pause-{index}")).exists() {
            let deadline = std::time::Instant::now() + Duration::from_secs(15);
            while !root.join(format!("release-{index}")).exists() {
                assert!(
                    std::time::Instant::now() < deadline,
                    "response barrier timed out"
                );
                std::thread::sleep(Duration::from_millis(5));
            }
        }
        if text.starts_with("DELETE /v1/workspaces/ws ") {
            let workspace = PathBuf::from(std::env::var_os("MANTLE_LIFECYCLE_WORKSPACE").unwrap());
            fs::remove_dir_all(workspace).unwrap();
        }
        stream
            .write_all(&fs::read(root.join(format!("response-{index}"))).unwrap())
            .unwrap();
    }
}
