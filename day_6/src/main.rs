extern crate ndarray;

use ndarray::prelude::*;

use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() -> Result<(), <Point as FromStr>::Err> {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let maxdist = args[2].parse::<i64>().unwrap();

    let contents = get_contents(filename);

    let points = get_points(&contents)?;

    println!("{}", largest_area(&points));
    println!("{}", close_area(&points, maxdist));

    Ok(())
}

fn close_area(points: &[Point], maxdist: i64) -> u64 {
    let size = get_max(points) + 1;

    let mut arena = Array::<i64, Ix2>::zeros((size, size));

    for i in 0..size {
        for j in 0..size {
            let mut dists: Vec<i64> = vec![0; points.len()];
            for (c, p) in points.iter().enumerate() {
                dists[c] = (p.x - i as i64).abs() + (p.y - j as i64).abs();
            }
            if dists.iter().sum::<i64>() >= maxdist {
                continue;
            }
            arena[[j, i]] = 1;
        }
    }
    arena.sum() as u64
}

fn largest_area(points: &[Point]) -> u64 {
    let size = get_max(points) + 1;

    let mut arena = Array::<i64, Ix2>::zeros((size, size));

    for i in 0..size {
        for j in 0..size {
            let mut dists = vec![0; points.len()];
            for (c, p) in points.iter().enumerate() {
                dists[c] = (p.x - i as i64).abs() + (p.y - j as i64).abs();
            }
            let mindist = dists.iter().min().unwrap();
            let mut noccs = 0;
            for dist in dists.iter() {
                if *dist == *mindist {
                    noccs += 1;
                }
            }
            if noccs > 1 {
                arena[[j, i]] = -1;
            } else {
                arena[[j, i]] = dists.iter().position(|&val| val == *mindist).unwrap() as i64;
            }
        }
    }

    let mut ignore: HashSet<usize> = HashSet::new();

    for i in 0..size {
        ignore.insert(arena[[0, i]] as usize);
        ignore.insert(arena[[size - 1, i]] as usize);
        ignore.insert(arena[[i, 0]] as usize);
        ignore.insert(arena[[i, size - 1]] as usize);
    }

    let mut arena_counts: HashMap<usize, u64> = HashMap::new();

    for c in (0..points.len()).filter(|x| !ignore.contains(x)) {
        for v in arena.iter() {
            if *v as usize == c {
                *arena_counts.entry(c).or_insert(0) += 1
            }
        }
    }

    *arena_counts.values().max().unwrap()
}

fn get_contents(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
}

fn get_points(contents: &str) -> Result<Vec<Point>, <Point as FromStr>::Err> {
    let mut points: Vec<Point> = Vec::new();

    for line in contents.lines() {
        let p = line.parse()?;
        points.push(p);
    }

    Ok(points)
}

fn get_max(points: &[Point]) -> usize {
    let mut max: i64 = 0;

    for p in points {
        if p.x > max {
            max = p.x
        }
        if p.y > max {
            max = p.y
        }
    }

    max as usize
}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').map(|p| p.trim()).collect();

        let x_fromstr = coords[0].parse::<i64>()?;
        let y_fromstr = coords[1].parse::<i64>()?;

        Ok(Point {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() -> Result<(), <Point as FromStr>::Err> {
        let contents = get_contents("test");
        let points = get_points(&contents)?;
        assert!(largest_area(&points) == 17);
        Ok(())
    }

    #[test]
    fn part2() -> Result<(), <Point as FromStr>::Err> {
        let contents = get_contents("test");
        let points = get_points(&contents)?;
        assert!(close_area(&points, 32) == 16);
        Ok(())
    }
}
