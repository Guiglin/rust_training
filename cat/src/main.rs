extern crate clap;

use clap::{Arg, App, AppSettings};
use std::fs::File;

fn main() {
    let matches = App::new("updatefw_rs")
        .version("1.0")
        .author("guiglin & nicolattuso")
        .about("implementation of cat in Rust!")
        .setting(AppSettings::TrailingVarArg)
        .arg(Arg::with_name("number")
             .short("n")
             .long("number")
             .help("number all output lines"))
        .arg(Arg::with_name("files")
             .multiple(true)
             .required(true))
        .get_matches();
    let files = matches.values_of("files").unwrap();
    let file_list: Vec<&str> = files.collect();
    println!("{}", file_list.join(" "));

    //let f = File::open(file_list[0]);
    //let mut buffer: Vec<String> = Vec::new();
//    f.read_to_end(&mut buffer);
//
//    for argument in &buffer {
//        println!("{}", argument);
//    }
}
