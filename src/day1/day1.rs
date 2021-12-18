use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, ErrorKind, Read};
use std::path::Path;

pub fn count_increment() {
    println!("This should count the number of times we see increments.");
    main();
    // File hosts must exist in current path before this produces output
    // if let Ok(lines) = read_lines("./src/day1/input.txt") {
    //     // Consumes the iterator, returns an (Optional) String
    //     for (i, line) in lines.enumerate() {
    //         if i == 0 {
    //             if let Ok(depth) = line {
    //                 println!("1 - {}: {}", i, depth);
    //             }
    //         }
    //     }
    // }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() -> Result<(), Error> {
    let vec = read(File::open("./src/day1/input.txt")?)?;
    // use `vec` for whatever
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
