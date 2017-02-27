use std::env;
use std::process;

fn main() {
    let tab: Vec<_> = env::args().collect();

    if tab.len() < 1 {
        println!("Needs at least one arg");
        process::exit(1);
    }

    let n: i64 = tab[1].parse().unwrap();
    let mut temp: i64;
    let mut i: i64 = 0;
    let mut j: i64 = 1;

    for k in 0..n {
        temp = i + j;
        i = j;
        j = temp;
        println!("{} {}", k, j);
    }
}
