extern crate ndarray;
#[macro_use]
extern crate itertools;

use ndarray::prelude::*;

use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
struct Rectangle {
    corner: Point,
    width: u64,
    height: u64,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let rectangles = get_rectangles(filename);
    let arena = get_arena(&rectangles);

    println!("{}", intersect(arena.clone()));
    println!("{}", no_intersect(arena.clone(), &rectangles));
}

fn get_contents(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents
}

fn get_rectangles(filename: &str) -> Vec<Rectangle> {
    let contents = get_contents(filename);
    let lines: Vec<&str> = contents.split('\n').collect();

    let mut rectangles: Vec<Rectangle> = Vec::new();

    for line in lines {
        let spl: Vec<&str> = line.split("@").collect();
        let pwh: Vec<&str> = spl[1].trim().split(':').collect();
        let p: &str = pwh[0];
        let wh: &str = pwh[1].trim();
        let xy: Vec<&str> = p.split(",").collect();
        let point: Point = Point {
            x: xy[0].parse::<u64>().unwrap(),
            y: xy[1].parse::<u64>().unwrap(),
        };
        let wh: Vec<&str> = wh.split("x").collect();
        let r: Rectangle = Rectangle {
            corner: point,
            width: wh[0].parse::<u64>().unwrap(),
            height: wh[1].parse::<u64>().unwrap(),
        };
        rectangles.push(r);
    }
    rectangles
}

fn get_arena(rectangles: &[Rectangle]) -> Array<u64, Ix2> {
    let mut arena = Array::<u64, Ix2>::zeros((1000, 1000));
    for r in rectangles {
        for i in 0..r.width {
            for j in 0..r.height {
                let x: u64 = r.corner.x + i;
                let y: u64 = r.corner.y + j;
                arena[[x as usize, y as usize]] += 1;
            }
        }
    }
    arena
}

fn no_intersect(arena: Array<u64, Ix2>, rectangles: &[Rectangle]) -> u64 {
    let mut count: u64 = 1;
    let mut skip: bool = false;
    for r in rectangles {
        for (i, j) in iproduct!(0..r.width, 0..r.height) {
            let x: u64 = r.corner.x + i;
            let y: u64 = r.corner.y + j;
            if arena[[x as usize, y as usize]] > 1 {
                skip = true
            }
            if skip {
                break;
            }
        }
        if !skip {
            println!("{:#?}", r);
            return count;
        }
        count += 1;
        skip = false;
    }
    unreachable!()
}

fn intersect(arena: Array<u64, Ix2>) -> u64 {
    let mut count = 0;
    for element in arena.iter() {
        if *element > 1 {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let rectangles = get_rectangles("test");
        let arena = get_arena(&rectangles);
        assert!(intersect(arena) == 4);
    }

    #[test]
    fn part2_example() {
        let rectangles = get_rectangles("test");
        let arena = get_arena(&rectangles);
        assert!(no_intersect(arena.clone(), &rectangles) == 3)
    }
}
