extern crate rayon;

use rayon::prelude::*;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = get_contents(filename);

    println!("{}", rayon::current_num_threads());

    let reacted_contents = react(contents);

    println!("{}", reacted_contents.len());
    println!("{}", best_react(reacted_contents));
}

fn get_contents(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
}

fn best_react(contents: String) -> usize {
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .par_chars()
        .map(|test| {
            let this_contents = contents.replace(test, "");
            let this_contents = this_contents.replace(test.to_ascii_lowercase(), "");
            react(this_contents).len()
        }).min()
        .unwrap()
}

fn react(mut result: String) -> String {
    result = result.trim_end().to_string();

    loop {
        let mut new_result: String = String::new();
        let mylen = result.len();
        {
            let mut iter = result.chars().peekable();
            while let Some(c1) = iter.next() {
                let mut skip = false;
                match iter.peek() {
                    Some(c2) => {
                        if (c1.to_ascii_lowercase() == c2.to_ascii_lowercase())
                            && ((c1.is_ascii_uppercase() && c2.is_ascii_lowercase())
                                || (c1.is_ascii_lowercase() && c2.is_ascii_uppercase()))
                        {
                            skip = true;
                        }
                    }
                    None => (),
                }
                if skip {
                    iter.next();
                } else {
                    new_result.push(c1);
                }
            }
        }
        if mylen == new_result.len() {
            break;
        }
        result = new_result;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let contents = get_contents("test");
        assert!(react(contents.clone()).len() == 10);
    }

    #[test]
    fn part2_example() {
        let contents = get_contents("test");
        assert!(best_react(contents.clone()) == 4);
    }
}
