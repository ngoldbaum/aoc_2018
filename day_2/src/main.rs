use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("{}", checksum(&filename));
}

fn checksum(filename: &str) -> i64 {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let lines = contents.lines();
    let mut num_two_times = 0;
    let mut num_three_times = 0;
    let mut has_two_times: bool;
    let mut has_three_times: bool;

    for line in lines {
        let mut unique_chars: HashMap<char, i64> = HashMap::new();
        for character in line.chars() {
            *unique_chars.entry(character).or_insert(0) += 1
        }

        has_two_times = false;
        has_three_times = false;
        for (_, count) in &unique_chars {
            if *count == 2 {
                has_two_times = true;
            }
            if *count == 3 {
                has_three_times = true
            }
        }
        if has_two_times {
            num_two_times += 1;
        }
        if has_three_times {
            num_three_times += 1;
        }
    }

    num_two_times * num_three_times
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert!(checksum("test") == 12);
    }
}
