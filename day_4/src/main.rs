use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let guards = get_guard_data(filename);

    println!("{}", guard_checksum(&guards));
    println!("{}", guard_frequent(&guards));
}

fn get_guard_data(filename: &str) -> HashMap<u64, Vec<u64>> {
    let contents = get_contents(filename);

    let mut contents: Vec<&str> = contents.split('\n').collect();

    contents.sort();

    let mut guards: HashMap<u64, Vec<u64>> = HashMap::new();

    let mut current_guard: u64 = 0;
    let mut iter = contents.iter();

    while let Some(line) = iter.next() {
        if line.contains("begins shift") {
            let beg_offset = line.find('#').unwrap_or(line.len()) + 1;
            let end_offset = line.find("begins").unwrap_or(line.len()) - 1;
            current_guard = line[beg_offset..end_offset].parse::<u64>().unwrap();
            continue;
        } else if line.contains("falls") {
            let offset = line.find("]").unwrap_or(line.len()) - 2;
            let begin = line[offset..offset + 2].parse::<u64>().unwrap();
            let line = iter.next().unwrap();
            let offset = line.find("]").unwrap_or(line.len()) - 2;
            let end = line[offset..offset + 2].parse::<u64>().unwrap();
            let guard_data = guards.entry(current_guard).or_insert(vec![0; 60]);
            for i in begin..end {
                guard_data[i as usize] += 1;
            }
        }
    }

    guards
}

fn guard_frequent(guards: &HashMap<u64, Vec<u64>>) -> u64 {
    let mut max_sleep: u64 = 0;
    let mut max_guard: u64 = 0;
    let mut max_sleep_minute: u64 = 0;

    for (guard, guard_data) in guards.iter() {
        for (i, slept_minute) in guard_data.iter().enumerate() {
            if *slept_minute > max_sleep {
                max_guard = *guard;
                max_sleep = *slept_minute;
                max_sleep_minute = i as u64;
            }
        }
    }

    max_sleep_minute * max_guard
}

fn guard_checksum(guards: &HashMap<u64, Vec<u64>>) -> u64 {
    let mut guard_ids: Vec<u64> = guards.keys().cloned().collect();
    guard_ids.sort();

    let mut guard_sums: HashMap<u64, u64> = HashMap::new();
    for (guard, guard_data) in guards.iter() {
        guard_sums.insert(*guard, guard_data.iter().sum());
    }

    let mut max_sum = 0;
    let mut max_guard = 0;
    for (guard, guard_sum) in guard_sums {
        if guard_sum > max_sum {
            max_guard = guard;
            max_sum = guard_sum;
        }
    }

    let guard_data = guards.get(&max_guard).unwrap();
    let mut max_minute: u64 = 0;
    let mut max_content: u64 = 0;
    for (i, item) in guard_data.iter().enumerate() {
        if *item > max_content {
            max_minute = i as u64;
            max_content = guard_data[i];
        }
    }

    max_guard * max_minute
}

fn get_contents(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let guards = get_guard_data("test");
        assert!(guard_checksum(&guards) == 240);
    }

    #[test]
    fn part2_example() {
        let guards = get_guard_data("test");
        assert!(guard_frequent(&guards) == 4455);
    }
}
