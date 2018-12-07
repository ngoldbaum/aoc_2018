use std::env;
use std::error;
use std::fs::File;
use std::io::prelude::*;

type Result<T> = std::result::Result<T, Box<error::Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = get_contents(filename)?;

    println!("{}", contents);

    let edges: Vec<(char, char)> = get_edges(&contents)?;

    println!("{:?}", edges);

    Ok(())
}

fn get_contents(filename: &str) -> Result<String> {
    let mut f = File::open(filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(contents)
}

fn get_edges(contents: &str) -> Result<Vec<(char, char)>> {
    let mut result: Vec<(char, char)> = Vec::new();

    for line in contents.lines() {
        let e1 = line
            .chars()
            .nth(line.find("Step ").ok_or("can't find 'Step '!")? + 5)
            .ok_or("can't index string")?;
        let e2 = line
            .chars()
            .nth(line.find("step ").ok_or("can't find 'step '!")? + 5)
            .ok_or("can't index string")?;
        result.push((e1, e2));
    }

    Ok(result)
}
