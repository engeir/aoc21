use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

pub fn count_increment() {
    println!("This should count the number of times we see increments.");
    if let Ok(_) = main() {}
}

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() -> Result<(), Error> {
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
    println!("{}", c);
    Ok(())
}
