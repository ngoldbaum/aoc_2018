use ndarray::prelude::*;

fn main() {
    let mut fuel_cell = ndarray::Array::<i64, Ix2>::zeros((299, 299));

    let args: Vec<String> = std::env::args().collect();

    let serial_number = &args[1]
        .parse::<usize>()
        .expect(&format!("Couldn't parse input {}", args[1]));

    for i in 1..300 {
        for j in 1..300 {
            let rack_id = i + 10;
            let power_level = (rack_id * j + serial_number) * rack_id;
            let power_level = power_level.to_string();
            let ndigits = power_level.len();
            let ipower_level: i64;
            if ndigits > 2 {
                ipower_level = power_level
                    .chars()
                    .nth(ndigits - 3)
                    .unwrap()
                    .to_string()
                    .parse::<i64>()
                    .unwrap();
            } else {
                ipower_level = 0;
            }
            fuel_cell[[i - 1, j - 1]] = (ipower_level as i64) - 5;
        }
    }

    let mut maxval = 0;
    let mut convolved = ndarray::Array::<i64, Ix2>::zeros((299, 299));

    for size in 3..100 {
        convolved.fill(0);
        for i in 1..(300 - size - 1) {
            for j in 1..(300 - size - 1) {
                let mut indices: Vec<(usize, usize)> = Vec::new();
                for k in 0..size {
                    for l in 0..size {
                        indices.push((i - 1 + k, j - 1 + l))
                    }
                }
                for index in indices.iter() {
                    convolved[[i - 1, j - 1]] += fuel_cell[*index];
                }
                if convolved[[i - 1, j - 1]] > maxval {
                    maxval = convolved[[i - 1, j - 1]];
                    println!("{},{},{}: {}", i, j, size, maxval);
                }
            }
        }
    }
}
