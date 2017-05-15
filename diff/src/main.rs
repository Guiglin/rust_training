use std::io::BufReader;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::env;

fn main() {
    let tab: Vec<_> = env::args().collect();

    println!("{:?}", &tab[1..]);

    assert_ne!(tab.len(), 2);

    let mut f = File::open(&tab[0]).unwrap();
    let mut f = BufReader::new(f);
    let mut f2 = File::open(&tab[1]).unwrap();
    let mut f2 = BufReader::new(f);
    let mut counter = 0;
    for line in f.lines() {
        if line != f2.lines() {
            counter +=1;
        }
    }
    println!("{} of differences between {:?}", counter, &tab[1..]);
}
