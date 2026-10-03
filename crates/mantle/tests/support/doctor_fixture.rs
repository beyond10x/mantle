//! Controlled provider/OpenSSH process protocols; never shipped as a CLI.
use std::{fs, io::{Read,Write}, os::unix::net::UnixListener, path::PathBuf, process::Command, time::Duration};
fn main() {
    std::thread::spawn(|| { std::thread::sleep(Duration::from_secs(30)); std::process::exit(124); });
    let root=PathBuf::from(std::env::var_os("MANTLE_DOCTOR_FIXTURE").unwrap());
    let args:Vec<_>=std::env::args().collect();
    if args.get(1).is_some_and(|a|a=="--descendant") { fs::write(root.join("descendant"),std::process::id().to_string()).unwrap(); loop {std::thread::park();} }
    let program=PathBuf::from(&args[0]); let program=program.file_name().unwrap().to_str().unwrap();
    let failure=fs::read_to_string(root.join("failure")).unwrap_or_default();
    fs::OpenOptions::new().append(true).create(true).open(root.join("calls")).unwrap().write_all(format!("{program}\n").as_bytes()).unwrap();
    if program=="credential" { fs::write(root.join("credential-read"),b"bad").unwrap(); std::process::exit(1); }
    if program=="aws" || program=="kubectl" || program=="virtctl" {
        fs::write(root.join("provider-argv"),args.join("\n")).unwrap();
        if failure=="timeout" { Command::new(std::env::current_exe().unwrap()).arg("--descendant").spawn().unwrap(); loop {std::thread::park();} }
        if failure=="provider" { eprintln!("PRIVATE_CANARY provider stderr"); std::process::exit(1); }
        print!("{}",fs::read_to_string(root.join("provider.json")).unwrap()); return;
    }
    assert_eq!(program,"ssh");
    assert!(args.contains(&"StrictHostKeyChecking=yes".into()));
    assert!(args.contains(&"UpdateHostKeys=no".into()));
    fs::write(root.join("ssh-argv"),args.join("\n")).unwrap();
    if failure=="ssh" { eprintln!("PRIVATE_CANARY ssh stderr"); std::process::exit(1); }
    if let Some(index)=args.iter().position(|a|a=="-L") {
        if failure=="tunnel-timeout" || failure=="discovery-timeout" || failure=="tunnel-exit" {
            Command::new(std::env::current_exe().unwrap()).arg("--descendant").spawn().unwrap();
            while !root.join("descendant").exists() {std::thread::sleep(Duration::from_millis(5));}
            if failure=="tunnel-exit" {std::process::exit(1);}
            if failure=="tunnel-timeout" {loop {std::thread::park();}}
        }
        let socket=args[index+1].split_once(':').unwrap().0;
        let listener=UnixListener::bind(socket).unwrap();
        let (mut stream,_)=listener.accept().unwrap(); stream.set_read_timeout(Some(Duration::from_secs(3))).unwrap();
        if failure=="discovery-timeout" {loop {std::thread::park();}}
        let mut header=Vec::new(); while !header.ends_with(b"\r\n\r\n") { let mut byte=[0];stream.read_exact(&mut byte).unwrap();header.push(byte[0]); assert!(header.len()<16384); }
        stream.write_all(&fs::read(root.join("discovery.http")).unwrap()).unwrap();
        loop {std::thread::park();}
    }
    let remote=args.last().unwrap();
    if remote=="true" { return; }
    if remote.contains("systemctl") { if failure=="service" { std::process::exit(1); } return; }
    if remote.contains("--version") { print!("{}",fs::read_to_string(root.join("versions")).unwrap());return; }
    panic!("unexpected remote command");
}
