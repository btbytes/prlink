# PRLink

Read all the Rust files in a directory and create
[play.rust-lang.org](http://play.rust-lang.org) links in Markdown
format.

That is, if a directory has a file `hello.rs` with the following
content:

```rust
fn main() {
    println!("Hello, world!");
}
```

Running `prlink` should result in :

```
- ["hello.rs"](http://play.rust-lang.org/?code=fn+main%28%29+%7B%0A++++println%21%28%22Hello%2C+world%21%22%29%3B%0A%7D%0A)
```

Click on the link below to see the source code of this console program
in rust playground.


- ["main.rs"](http://play.rust-lang.org/?code=%2F%2F+%23%21%5Bfeature%28plugin%29%5D%0A%2F%2F+%23%21%5Bplugin%28clippy%29%5D%0A%0A%0Aextern+crate+clap%3B%0Aextern+crate+glob%3B%0Aextern+crate+url%3B%0A%0Ause+clap%3A%3A%7BApp%2C+Arg%7D%3B%0Ause+std%3A%3Aerror%3A%3AError%3B%0Ause+std%3A%3Afs%3A%3AFile%3B%0Ause+std%3A%3Aio%3A%3Aprelude%3A%3A*%3B%0Ause+url%3A%3Aform_urlencoded%3A%3A%7Bserialize%7D%3B%0Ause+glob%3A%3Aglob%3B%0Ause+std%3A%3Apath%3A%3APathBuf%3B%0A%0A%0Afn+read_file%28path%3A+String%29+-%3E+String+%7B%0A++++let+mut+file+%3D+match+File%3A%3Aopen%28%26path%29+%7B%0A++++++++Err%28why%29+%3D%3E+panic%21%28%22couldn%27t+open+%7B%7D%3A+%7B%7D%22%2C+path%2C+Error%3A%3Adescription%28%26why%29%29%2C%0A++++++++Ok%28file%29+%3D%3E+file%2C%0A++++%7D%3B%0A++++let+mut+s+%3D+String%3A%3Anew%28%29%3B%0A++++match+file.read_to_string%28%26mut+s%29+%7B%0A++++++++Err%28why%29+%3D%3E+panic%21%28%22couldn%27t+read+%7B%7D%3A+%7B%7D%22%2C+path%2C+Error%3A%3Adescription%28%26why%29%29%2C%0A++++++++Ok%28_%29+%3D%3E+s%2C%0A++++%7D%0A%7D%0A%0A%0Afn+main%28%29+%7B%0A++++let+matches+%3D+App%3A%3Anew%28%22prlink%22%29%0A++++++++.version%28%221.0%22%29%0A++++++++.author%28%22Pradeep+Gowda+%3Cpradeep%40btbytes.com%3E%22%29%0A++++++++.about%28%22Print+Rust+playground+links%22%29%0A++++++++.arg%28Arg%3A%3Awith_name%28%22INPUT%22%29%0A+++++++++++++.help%28%22Sets+the+input+path+to+use%22%29%0A+++++++++++++.required%28true%29%0A+++++++++++++.index%281%29%29%0A++++++++.get_matches%28%29%3B%0A++++let+dirpath+%3D+matches.value_of%28%22INPUT%22%29.unwrap%28%29%3B%0A++++let+relative_path+%3D+PathBuf%3A%3Afrom%28dirpath%29%3B%0A++++let+mut+absolute_path+%3D+std%3A%3Aenv%3A%3Acurrent_dir%28%29.unwrap%28%29%3B%0A++++absolute_path.push%28relative_path%29%3B%0A++++absolute_path.push%28%22*.rs%22%29%3B%0A%0A++++for+entry+in+glob%28absolute_path.to_str%28%29.unwrap%28%29%29.unwrap%28%29+%7B%0A++++++++match+entry+%7B%0A++++++++++++Ok%28path%29+%3D%3E+%7B%0A++++++++++++++++let+data+%3D+%26%5B%28%22code%22.to_string%28%29%2C+read_file%28path.to_str%28%29.unwrap%28%29.to_string%28%29%29%29%5D%3B%0A++++++++++++++++let+s+%3D+serialize%28data%29%3B%0A++++++++++++++++let+fname+%3D+path.to_str%28%29.unwrap%28%29.to_string%28%29.split%28%27%2F%27%29.last%28%29.unwrap%28%29.to_string%28%29%3B%0A++++++++++++++++println%21%28%22-+%5B%7B%3A%3F%7D%5D%28http%3A%2F%2Fplay.rust-lang.org%2F%3F%7B%7D%29%22%2C+fname%2C+s%29%3B%0A++++++++++++%7D%2C%0A++++++++++++Err%28e%29+%3D%3E+println%21%28%22%7B%3A%3F%7D%22%2C+e%29%2C%0A++++++++%7D%0A++++%7D%0A%7D%0A)


The above link was generated by running:

```shell
./target/debug/prlink ./src
```

**Update 2015/9/17**: I wrote this program essentially to make it easy for [https://github.com/carols10cents/rustlings](https://github.com/carols10cents/rustlings) to create links to the Rust Playground.  And [Carol](https://github.com/carols10cents) is using it. Yay!
