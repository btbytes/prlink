#![feature(plugin)]
#![plugin(clippy)]

extern crate clap;
extern crate glob;
extern crate prlink;

use clap::{App, Arg};
use glob::glob;
use std::path::PathBuf;

fn main() {
    let matches = App::new("prlink")
        .version("1.0")
        .author("Pradeep Gowda <pradeep@btbytes.com>")
        .about("Print Rust playground links")
        .arg(Arg::with_name("INPUT")
             .help("Sets the input path to use")
             .required(true)
             .index(1))
        .get_matches();
    let dirpath = matches.value_of("INPUT").unwrap();
    let relative_path = PathBuf::from(dirpath);
    let mut absolute_path = std::env::current_dir().unwrap();
    absolute_path.push(relative_path);
    absolute_path.push("*.rs");

    for entry in glob(absolute_path.to_str().unwrap()).unwrap() {
        match entry {
            Ok(path) => {
                println!("{}", prlink::linkify_file(&path));
            },
            Err(e) => println!("{:?}", e),
        }
    }
}
