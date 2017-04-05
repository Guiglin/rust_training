use std::env;
use std::io::prelude::*;
use std::net::UdpSocket;
use std::fs::File;

fn main() {
    println!("Start Download Manager");

    //let tab: Vec<_> = env::args().collect();
    //assert_ne!(tab.len(), 1);
    let mut file = File::create("received_file").unwrap();
    let stream = UdpSocket::bind("127.0.0.1:8080").expect("could not bind to address");
    println!("Connected to the host");
    let mut buf = vec![] ;
    println!("Start downloading the file");
    let result = stream.recv_from(&mut buf);
    drop(stream);

    match result {
        Ok ((size, src)) => {
            println!("Received data from {}", src);
            println!("Size received = {}", size);
        },
        Err(error) => println!("{}", error.to_string()),
    }
    println!("Write the downloaded file into the disk");
    file.write_all(&mut buf).unwrap();
}
