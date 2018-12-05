use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = get_contents(filename);
    
    println!("{}", react(contents.clone()));
    println!("{}", best_react(contents.clone()));
}

fn get_contents(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
}

fn best_react(contents: String) -> usize {
    let mut best_len = contents.len();
    
    for test in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
        let this_contents = contents.replace(test, "");
        let this_contents = this_contents.replace(test.to_ascii_lowercase(), "");

        let react_len = react(this_contents);
        
        if react_len < best_len {
            println!("{}, {}", test, react_len);
            best_len = react_len;
        }
    }

    best_len
}

fn react(mut result: String) -> usize {
    result = result.trim_end().to_string();
    
    loop {
        let mut i = 0;
        {
            let mut iter = result.chars().peekable();
            while let Some(c1) = iter.next() {
                let mut skip = false;
                match iter.peek() {
                    Some(c2) => {
                        if (c1.to_ascii_lowercase() == c2.to_ascii_lowercase()) && 
                            ((c1.is_ascii_uppercase() && c2.is_ascii_lowercase()) ||
                             (c1.is_ascii_lowercase() && c2.is_ascii_uppercase())) {
                                skip = true;
                            }
                    }
                    None => (),
                }
                if skip {
                    break;
                } else {
                    i += 1;
                }
            }
        }
        if i == result.len() {
            break;
        }
        result.remove(i);
        result.remove(i);
    }
    result.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let contents = get_contents("test");
        assert!(react(contents.clone()) == 10);
    }

    #[test]
    fn part2_example() {
        let contents = get_contents("test");
        assert!(best_react(contents.clone()) == 4);
    }
}

    
