use std::env;
use std::error;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

use ndarray::prelude::*;

type Result<T> = std::result::Result<T, Box<error::Error>>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = get_contents(filename)?;

    let (mut positions, velocities) = parse_contents(&contents)?;
    let mut minx: i64;
    let mut miny: i64;
    let mut maxx: i64;
    let mut maxy: i64;
    let mut oldarea = 0;
    let mut time = 0;

    loop {
        for (p, v) in positions.iter_mut().zip(velocities.iter()) {
            p.x += v.x;
            p.y += v.y;
        }

        minx = positions.iter().map(|p| p.x).min().unwrap();
        miny = positions.iter().map(|p| p.y).min().unwrap();
        maxx = positions.iter().map(|p| p.x).max().unwrap();
        maxy = positions.iter().map(|p| p.y).max().unwrap();

        let area = (maxy - miny) * (maxx - minx);

        if area > oldarea && oldarea != 0 {
            break;
        }

        oldarea = area;

        time += 1;

        if area < 1000 {
            let mut arena = ndarray::Array::<i64, Ix2>::zeros((
                (maxy - miny + 1) as usize,
                (maxx - minx + 1) as usize,
            ));

            for p in positions.iter() {
                arena[((p.y - miny) as usize, (p.x - minx) as usize)] = 1;
            }

            println!("{:?}", arena);
            println!("{}", time);
        }
    }

    Ok(())
}

#[derive(Debug)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Velocity {
    x: i64,
    y: i64,
}

impl FromStr for Position {
    type Err = Box<error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let s = s[s.rfind("position=<").ok_or("can't find position")? + 10
            ..s.find("> velocity").ok_or("can't find velocity")?]
            .to_string();
        let coords: Vec<&str> = s.split(',').map(|p| p.trim()).collect();

        let x_fromstr = coords[0].parse::<i64>()?;
        let y_fromstr = coords[1].parse::<i64>()?;

        Ok(Position {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}

impl FromStr for Velocity {
    type Err = Box<error::Error>;

    fn from_str(s: &str) -> Result<Self> {
        let s =
            s[s.rfind("velocity=<").ok_or("can't find velocity")? + 10..s.len() - 1].to_string();
        let coords: Vec<&str> = s.split(',').map(|p| p.trim()).collect();

        let x_fromstr = coords[0].parse::<i64>()?;
        let y_fromstr = coords[1].parse::<i64>()?;

        Ok(Velocity {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}

fn parse_contents(contents: &str) -> Result<(Vec<Position>, Vec<Velocity>)> {
    let mut positions: Vec<Position> = Vec::new();
    let mut velocities: Vec<Velocity> = Vec::new();
    for line in contents.lines() {
        positions.push(line.parse()?);
        velocities.push(line.parse()?)
    }

    Ok((positions, velocities))
}

fn get_contents(filename: &str) -> Result<String> {
    let mut f = File::open(filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(contents)
}
