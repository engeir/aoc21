use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

/// Read a file and return a Vec of its lines
fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

/// Run the two increment functions for day 1 of advent of code.
pub fn count_increment() {
    if let Ok(_) = count_single_increment() {}
    if let Ok(_) = count_three_step_increment() {}
}

/// Counts the number of times a number is larger than the previous one.
fn count_single_increment() -> Result<(), Error> {
    let vec = read(File::open("./src/day1/input.txt")?)?;
    let mut i = 0;
    let mut c = 0;
    let mut prev = 0;
    for v in vec {
        if i != 0 {
            if v > prev {
                c += 1
            }
        }
        i += 1;
        prev = v
    }
    println!("Single step increments: {}", c);
    Ok(())
}

/// Counts the number of times three consecutive numbers are increasing.
fn count_three_step_increment() -> Result<(), Error> {
    let vec = read(File::open("./src/day1/input.txt")?)?;
    let mut i = 0;
    let mut p1 = 0;
    let mut p2 = 0;
    let mut p3 = 0;
    let mut d = 0;
    for v in vec {
        if i > 2 {
            let a = [p3, p2, p1];
            let b = [p2, p1, v];
            let sum_a: i64 = a.iter().sum();
            let sum_b: i64 = b.iter().sum();
            if sum_b > sum_a {
                d += 1
            }
        }
        p3 = p2;
        p2 = p1;
        p1 = v;
        i += 1;
    }
    println!("Three step increments: {}", d);
    Ok(())
}
