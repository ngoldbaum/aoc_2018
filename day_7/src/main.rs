use std::env;
use std::fs::File;
use std::io::prelude::*;


fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = get_contents(filename);
    
    println!("{}", contents);
}

fn get_contents(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
}
