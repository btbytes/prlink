#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate url;

use url::form_urlencoded::Serializer;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn linkify_file(path: &PathBuf) -> String {
    let data = &[("code".to_owned(), read_file(path.to_str().unwrap()))];
    let s = Serializer::new(String::new())
        .extend_pairs(data)
        .finish();
    let fname = path.file_name().unwrap().to_string_lossy();
    format!("- [{:?}](https://play.rust-lang.org/?{})", fname, s)
}

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
