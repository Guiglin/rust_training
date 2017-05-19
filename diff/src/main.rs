extern crate difference;
use difference::{Difference, Changeset};
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::env;

fn main() {
    let tab: Vec<_> = env::args().collect();

    println!("{:?}", &tab[1..]);

    assert_ne!(tab.len(), 2);

    let mut f = File::open(&tab[1]).unwrap();
    let mut buffer = String::new();
    match f.read_to_string(&mut buffer) {
        Err(why) => panic!("Couldn't read {}: {}", &tab[0], why.description()),
        Ok(_) => println!("File {} read!", &tab[0]),
    }
    let mut f2 = File::open(&tab[2]).unwrap();
    let mut buffer2 = String::new();
    match f2.read_to_string(&mut buffer2) {
        Err(why) => panic!("Couldn't read {}: {}", &tab[1], why.description()),
        Ok(_) => println!("File {} read!", &tab[1]),
    };

    let Changeset { diffs, .. } = Changeset::new(buffer.as_ref(), buffer2.as_ref(), " ");

    for i in 0..diffs.len() {
        match diffs[i] {
            Difference::Same(ref x) => {
                println!("{}", x);
            }
            Difference::Add(ref x) => {
                println!("+{}", x);
            }
            Difference::Rem(ref x) => {
                println!("-{}", x);
            }
        }
    }
}
