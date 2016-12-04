extern crate walkdir;

use std::env;
use std::fs::File;
use std::io::Error;
use std::io::Read;
use std::path::Path;
use walkdir::WalkDir;

fn read_file<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}

fn main() {

    let out_dir = env::var("OUT_DIR").unwrap();

    for entry in WalkDir::new("src/shaders").into_iter().filter_map(|e| e.ok()) {
        
        let metadata = entry.metadata().unwrap();

        if metadata.is_file() {
            println!("{}", read_file(entry.path()).unwrap());
        }
    }   


    panic!();
}