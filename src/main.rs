#![feature(plugin)]
#![plugin(clippy)]


extern crate clap;
extern crate glob;
extern crate url;

use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use url::form_urlencoded::Serializer;
use glob::glob;
use std::path::PathBuf;


fn read_file(path: &str) -> String {
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path, Error::description(&why)),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", path, Error::description(&why)),
        Ok(_) => s,
    }
}

fn linkify_file(path: &PathBuf) -> String {
    let data = &[("code".to_owned(), read_file(path.to_str().unwrap()))];
    let s = Serializer::new(String::new())
        .extend_pairs(data)
        .finish();
    let fname = path.file_name().unwrap().to_string_lossy();
    format!("- [{:?}](https://play.rust-lang.org/?{})", fname, s)
}

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
                println!("{}", linkify_file(&path));
            },
            Err(e) => println!("{:?}", e),
        }
    }
}
