use std::env;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    println!("Start Download Manager");

    //let tab: Vec<_> = env::args().collect();
    //assert_ne!(tab.len(), 1);
    let file = File::create("received_file");
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    let mut buf = vec![] ;
    let _ = stream.read_to_end(&mut buf).unwrap();
}
