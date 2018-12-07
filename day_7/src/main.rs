use std::env;
use std::fs::File;
use std::io::prelude::*;


fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = get_contents(filename)?;

    println!("{}", contents);
    
    Ok(())
}

fn get_contents(filename: &str) -> Result<String, std::io::Error> {
    let mut f = File::open(filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(contents)
}
