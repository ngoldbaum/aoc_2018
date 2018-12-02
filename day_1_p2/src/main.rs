use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("{}", get_double(&filename))
}

fn get_double(filename: &str) -> i64 {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    
    let lines = contents.lines();

    let mut frequency: i64 = 0;
    let mut unique_frequencies: HashSet<i64> = HashSet::new();

    for line in lines.cycle() {
        if unique_frequencies.contains(&frequency) {
            break;
        }
        unique_frequencies.insert(frequency);
        frequency += line.parse::<i64>().unwrap();
    }

    frequency
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn examples() {
        assert!(get_double("test1") == 0);
        assert!(get_double("test2") == 10);
        assert!(get_double("test3") == 5);
        assert!(get_double("test4") == 14);
    }
}
