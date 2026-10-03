//! Controlled OpenSSH wire seam; every SDK request remains parsed by the shipped CLI.
use std::{
    fs,
    io::{Read, Write},
    os::unix::net::UnixListener,
    path::PathBuf,
    time::Duration,
};
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let index = args.iter().position(|s| s == "-L").unwrap();
    let listener = UnixListener::bind(args[index + 1].split_once(':').unwrap().0).unwrap();
    let root = PathBuf::from(std::env::var_os("METADATA_FIXTURE").unwrap());
    for index in 0..3 {
        let (mut stream, _) = listener.accept().unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        let mut request = Vec::new();
        while !request.ends_with(b"\r\n\r\n") {
            let mut byte = [0];
            stream.read_exact(&mut byte).unwrap();
            request.push(byte[0]);
            assert!(request.len() < 65536);
        }
        let text = String::from_utf8(request.clone()).unwrap();
        assert!(!text.contains("/output"));
        fs::write(root.join(format!("request-{index}")), request).unwrap();
        stream
            .write_all(&fs::read(root.join(format!("response-{index}"))).unwrap())
            .unwrap();
    }
    std::thread::sleep(Duration::from_secs(30));
}
