use std::fs;
use std::env;
use std::path::Path;

fn main() {
    let tab: Vec<_> = env::args().collect();

    if tab.len()==1 {
        for entry in fs::read_dir("./").unwrap() {
            match entry {
                Ok(n) => println!("> {:?}", n.path()),
                Err(err) => println!("Error: {}", err),
            }
        }
    } else {
        for n in tab[1..].iter() {
            let dir = Path::new(n);
            if dir.is_dir() {
                for entry in fs::read_dir(dir).unwrap() {
                    match entry {
                        Ok(i) => println!("> {:?}", i.path()),
                        Err(err) => println!("Err: {}", err),
                    }
                }
            } else {
                println!("{:?} unknown file or directory", dir);
            }
        }
    }
}
