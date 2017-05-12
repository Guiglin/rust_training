use std::io::BufReader;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::env;

fn main() {
    let tab: Vec<_> = env::args().collect();
    let mut file_tab: Vec<_> = Vec::new();
    let mut iterator: usize = 0;
    let mut v: Vec<&str> = Vec::new();
    let mut buffer = String::new();
    let mut buffer2 = String::new();

    println!("{:?}", &tab[1..]);

    assert_ne!(tab.len(), 2);

    for n in &tab {
        let f = File::open(&n).unwrap();
        let f = BufReader::new(f);
        file_tab[iterator] = f;
        iterator += 1;
    }
    file_tab[0].read_to_string(&mut buffer);
    file_tab[1].read_to_string(&mut buffer2);
}
