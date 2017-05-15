extern crate difference;
use difference::Changeset;
use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    let tab: Vec<_> = env::args().collect();

    println!("{:?}", &tab[1..]);

    assert_ne!(tab.len(), 2);

    let mut f = File::open(&tab[0]).unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    let mut f2 = File::open(&tab[1]).unwrap();
    let mut buffer2 = String::new();
    f2.read_to_string(&mut buffer2).unwrap();

    let changeset = Changeset::new(buffer.as_ref(), buffer2.as_ref(), "");

    println!("{}", changeset);
}
