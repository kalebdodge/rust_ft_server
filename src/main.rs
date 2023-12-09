pub mod message;
use crate::message::{
    Message, MessageKind, Command
};
use std::{
    io::{ prelude::*, },
    net::{ TcpStream, TcpListener, },
    fs,
    env, process,
};
use serde::Deserialize;

fn main() -> std::io::Result<()> {
    //let addr = format!("{}:{}", _get_lan_addr(), _get_port());
    let addr = String::from("127.0.0.1:8080");
    println!("SERVER LISTENING AT: {}", &addr);
    let listener = TcpListener::bind(addr).unwrap();
    for stream in listener.incoming() {
        println!("\nINCOMING CONNECTION");
        let reader_stream = stream.unwrap();
        let writer_stream = reader_stream.try_clone().unwrap();
        let message = read_from_stream(reader_stream);

        // HANDLE MESSAGE
        match message.command {
            Command::NA => println!("NO COMMAND"),
            Command::Store(path) => {
                if let MessageKind::File(f) = message.kind {
                    store_file(&format!("{}{}", path, f),message.contents);
                }
            },
            Command::Read => {
                println!("{}", message.contents);
            },
        }
        write_to_stream(writer_stream, Message::_empty_message());
    }
    return Ok(());
}

fn read_from_stream(mut stream: TcpStream) -> Message {
    let mut buffer = serde_json::Deserializer::from_reader(stream);
    let message = Message::deserialize(&mut buffer).unwrap();
    return message;
}

fn write_to_stream(mut stream: TcpStream, message: Message) {
    let written = stream.write(
        serde_json::to_string(&message)
        .unwrap()
        .as_bytes()
    );
    println!("{} BYTES WRITTEN TO STREAM", written.unwrap());
}

fn _get_lan_addr() -> String {
    let output = process::Command::new("hostname")
        .arg("-I")
        .output()
        .expect("FAILED TO EXECUTE PROCESS \"hostname -I\"");
    let mut addr: String = String::from_utf8(output.stdout).unwrap();
    while addr.contains("\n") || addr.contains(" ") {
        addr.pop();
    }
    return addr;
}

fn _get_port() -> String {
    let mut port: String = String::new();
    for arg in env::args() {
        if arg.contains("target/debug") {
            continue;
        }
        port = String::from(arg);
    }
    return port;
}

fn store_file(path: &str, contents: String) {
    println!("{:#?}", path);
    //println!("{}", contents);
    fs::write(path, contents.as_str());
}
