use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::env;

fn main() {

    let tab: Vec<_> = env::args().collect();

    println!("{:?}", &tab[1..]);

    assert_ne!(tab.len(), 1);

    for n in tab[1..].iter() {
        let f = File::open(&n).unwrap();
        let f = BufReader::new(f);

        for line in f.lines() {
            println!("{}", line.unwrap());
        }
    }
}
