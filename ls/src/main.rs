use std::fs;
use std::env;

fn main() {
    let tab: Vec<_> = env::args().collect();

    if tab.len()==1 {
        for entry in fs::read_dir("./").unwrap() {
            match entry {
                Ok(n) => println!("> {:?}", n.path()),
                Err(err) => println!("Error: {}", err),
            }
        }
    }
}
