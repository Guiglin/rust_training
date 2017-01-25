use std::env;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    println!("Start Download Manager");

    //let tab: Vec<_> = env::args().collect();
    //assert_ne!(tab.len(), 1);
    let mut file = File::create("received_file").unwrap();
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    println!("Connected to the host");
    let mut buf = vec![] ;
    println!("Start downloading the file");
    let _ = stream.read_to_end(&mut buf).unwrap();
    println!("Write the downloaded file into the disk");
    file.write_all(&mut buf).unwrap();
}
