use std::fs;

fn read(io: &str) -> String {
    let br = format!("{}", io);
    fs::read_to_string(&br).unwrap_or_else(|_| panic!("Error reading file {}", br))
}

pub fn diagnostic_power_consumption() {
    let data = read("./src/day3/input.txt");
    let mut chars = 0;
    for i in data.lines() {
        chars = i.chars().count();
        break
    }

    let mut gamma: String = "".to_owned();
    let mut epsilon: String = "".to_owned();
    for c in 0..chars {
        let mut ones = 0;
        let mut n = 0;
        for i in data.lines() {
            n += 1;
            let ch = i.chars().nth(c).unwrap();
            if ch == '1' {
                ones += 1;
            }
        }
        if ones > n / 2 {
            gamma.push_str("1");
            epsilon.push_str("0");
        } else {
            gamma.push_str("0");
            epsilon.push_str("1");
        }
    }
    let g = isize::from_str_radix(&gamma, 2).unwrap();
    let e = isize::from_str_radix(&epsilon, 2).unwrap();
    println!("Gamma: {}; Epsilon: {}", gamma, epsilon);
    println!("Gamma: {}; Epsilon: {}", g, e);
    println!("Multiplied: {}", g * e);
}

fn diagnostic_oxygen() -> String {
    let data = read("./src/day3/input.txt");
    let mut oxy = vec![];
    let mut chars = 0;
    for i in data.lines() {
        chars = i.chars().count();
        let mut v = vec![i];
        oxy.append(&mut v);
    }
    let mut c = 0;
    loop {
        let mut n = 0;
        let mut ones = 0;
        let mut key = 1;
        for i in oxy.iter() {
            n += 1;
            let ch = i.chars().nth(c).unwrap();
            if ch == '1' {
                ones += 1;
            }
        }
        if ones * 2 < n {
            key = 0;
        }
        let the_key = key.to_string();
        oxy.retain(|&x| x.chars().nth(c).unwrap().to_string() == the_key);
        c += 1;
        c = (c).rem_euclid(chars);
        if oxy.len() == 1 {
            break
        }
    }
    oxy.into_iter().nth(0).unwrap().to_string()
}

fn diagnostic_co2() -> String {
    let data = read("./src/day3/input.txt");
    let mut oxy = vec![];
    let mut chars = 0;
    for i in data.lines() {
        chars = i.chars().count();
        let mut v = vec![i];
        oxy.append(&mut v);
    }
    let mut c = 0;
    loop {
        let mut n = 0;
        let mut ones = 0;
        let mut key = 1;
        for i in oxy.iter() {
            n += 1;
            let ch = i.chars().nth(c).unwrap();
            if ch == '1' {
                ones += 1;
            }
        }
        if ones * 2 >= n {
            key = 0;
        }
        if ones == 0 {
            key = 0;
        }
        if ones == n {
            key = 1;
        }
        let the_key = key.to_string();
        oxy.retain(|&x| x.chars().nth(c).unwrap().to_string() == the_key);
        c += 1;
        c = (c).rem_euclid(chars+1);
        if oxy.len() == 1 {
            break
        }
    }
    oxy.into_iter().nth(0).unwrap().to_string()
}

pub fn diagnostic_life_support() {
    let oxygen = diagnostic_oxygen();
    let co2 = diagnostic_co2();
    let o = isize::from_str_radix(&oxygen, 2).unwrap();
    let c = isize::from_str_radix(&co2, 2).unwrap();
    println!("Oxygen: {}; CO2: {}", oxygen, co2);
    println!("Oxygen: {}; CO2: {}", o, c);
    println!("Multiplied: {}", o * c);
}
