use std::env;
use std::fs::File;

fn main() {

    let tab: Vec<_> = env::args().collect();

    println!("Args are :");
    for argument in &tab {
        println!("{}", argument);
    }
    if tab.len() > 1 {
        println!("arg tab[1] = {}", tab[1]);
    }

    assert_ne!(tab.len(), 1);
    let path;
    path = &tab[1];

    let f = File::open(path);
    let mut buffer = Vec::new();

//    f.read_to_end(&mut buffer);
//
//    for argument in &buffer {
//        println!("{}", argument);
//    }
}
