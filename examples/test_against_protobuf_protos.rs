extern crate protobuf_parser;

use std::path::Path;
use std::fs;
use std::env;
use std::io::Read;

fn parse_recursively(path: &Path) {
    let file_name = path.file_name().expect("file_name").to_str().expect("to_str");
    if path.is_dir() {
        for entry in fs::read_dir(path).expect("read_dir") {
            parse_recursively(&entry.expect("entry").path());
        }
    } else if file_name.ends_with(".proto") {
        println!("checking {}", path.display());
        let mut content = String::new();
        fs::File::open(path).expect("open").read_to_string(&mut content).expect("read");
        protobuf_parser::FileDescriptor::parse(&content).expect("parse");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(2, args.len());
    // arg[1] is a path to protobuf tree
    parse_recursively(&Path::new(&args[1]));
}
