use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::env;

fn main() {
    let mut tab: Vec<_> = env::args().collect();

    if tab.len()==1 {
        tab[1] = "./".to_string();
    }
}
