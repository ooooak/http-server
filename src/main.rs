use std::env;
use std::process;
use std::thread;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::BufReader;

mod headers;
mod file_io;
mod response;


fn main() {
    match env::args().nth(1) {
        Some(_) => (),
        None => {
            println!("{}", "Please specify the path of root directory.");
            process::exit(0)
        },
    }

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    loop {
        match listener.accept() {
            Ok((stream, _)) => {  
                thread::spawn(move || { 
                    handle_connection(stream)
                });
            },
            Err(e) => {  
                thread::spawn(move || { 
                    println!("Connection failed: {:?}", e); 
                });
            },
        };
    };
}

/* from https://github.com/PritiKumr/rust-httpd 
*/
fn read_request_head(stream: &TcpStream) -> Vec<u8> {
    let mut reader = BufReader::new(stream);
    let mut buff = Vec::new();
    let mut read_bytes = reader.read_until(b'\n', &mut buff).unwrap();
    while read_bytes > 0 {
        read_bytes = reader.read_until(b'\n', &mut buff).unwrap();
        if read_bytes == 2 && &buff[(buff.len()-2)..] == b"\r\n" {
            break;
        }
    }
    return buff;
}

fn handle_connection(mut stream: TcpStream) {
    let buff = read_request_head(&stream);
    let resp = response::create(buff);

    // custom press
    match stream.write(&resp) {
        Ok(_) => (),
        Err(_) => (),
    };

    stream.flush().expect("flush failed");
}

