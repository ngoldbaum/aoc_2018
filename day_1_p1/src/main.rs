use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("{}", get_frequency(&filename))
}

fn get_frequency(filename: &str) -> i64 {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let lines = contents.lines();

    let mut frequency: i64 = 0;

    for line in lines {
        frequency += line.parse::<i64>().unwrap();
    }

    frequency
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert!(get_frequency("test1") == 3);
        assert!(get_frequency("test2") == 0);
        assert!(get_frequency("test3") == -6);
    }
}
