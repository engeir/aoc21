use std::fs;

fn read(io: &str) -> String {
    let br = format!("{}", io);
    fs::read_to_string(&br).unwrap_or_else(|_| panic!("Error reading file {}", br))
}

pub fn track_position() {
    let data = read("./src/day2/input.txt");
    let mut dep = 0;
    let mut fwd = 0;
    for i in data.lines() {
        let spl = i.split(" ").collect::<Vec<&str>>();
        match spl[0] {
            "forward" => fwd += spl[1].parse::<i32>().unwrap(),
            "down" => dep += spl[1].parse::<i32>().unwrap(),
            "up" => dep -= spl[1].parse::<i32>().unwrap(),
            _ => panic!(),
        }
    }
    println!("Depth change: {}, Forward change: {}", dep, fwd);
    println!("Multiplied: {}", dep * fwd);
}

pub fn track_position_aim() {
    let data = read("./src/day2/input.txt");
    let mut aim = 0;
    let mut dep = 0;
    let mut fwd = 0;
    for i in data.lines() {
        let spl = i.split(" ").collect::<Vec<&str>>();
        let val = spl[1].parse::<i32>().unwrap();
        match spl[0] {
            "forward" => {
                fwd += val;
                dep += val * aim;
            }
            "down" => aim += val,
            "up" => aim -= val,
            _ => panic!(),
        }
    }
    println!(
        "Depth change: {}, Forward change: {}, Aim change: {}",
        dep, fwd, aim
    );
    println!("Multiplied: {}", dep * fwd);
}
