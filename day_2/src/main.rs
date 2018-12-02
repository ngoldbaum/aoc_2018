#[macro_use]
extern crate itertools;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("{}", checksum(&filename));
    println!("{}", one_off(&filename));
}

fn get_contents(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
}

fn one_off(filename: &str) -> String {
    let contents = get_contents(filename);
    let lines: Vec<&str> = contents.split('\n').collect();

    let mut ret: Vec<char> = Vec::new();

    for (word1, word2) in iproduct!(&lines, &lines) {
        let mut num_diff = 0;
        for (ch1, ch2) in izip!(word1.chars(), word2.chars()) {
            if ch1 != ch2 {
                num_diff += 1;
            }
        }
        if num_diff == 1 {
            for (ch1, ch2) in izip!(word1.chars(), word2.chars()) {
                if ch1 == ch2 {
                    ret.push(ch1);
                }
            }
            break;
        }
    }

    String::from_iter(ret.to_owned())
}

fn checksum(filename: &str) -> i64 {
    let contents = get_contents(filename);

    let mut num_two_times: i64 = 0;
    let mut num_three_times: i64 = 0;
    let mut has_two_times: bool;
    let mut has_three_times: bool;

    for line in contents.lines() {
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
    fn part1_example() {
        assert!(checksum("test") == 12);
    }

    #[test]
    fn part2_example() {
        assert!(one_off("test2") == "fgij");
    }
}
