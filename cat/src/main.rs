use std::error::Error;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use std::env;

fn main() {

    let tab: Vec<_> = env::args().collect();

    println!("{:?}", &tab[1..]);
    if tab.len() > 1 {
        println!("arg tab[1] = {}", tab[1]);
    }

    assert_ne!(tab.len(), 1);
    let path;
    path = &tab[1];

    let f = File::open(&path).unwrap();
    let f = BufReader::new(f);

    for line in f.lines() {
        println!("{}", line.unwrap());
    }
}
