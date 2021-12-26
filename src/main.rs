mod day1;
mod day2;
mod day3;

fn main() {
    println!("Day 1:");
    day1::day1::count_increment();
    println!("");
    println!("Day 2:");
    day2::day2::track_position();
    day2::day2::track_position_aim();
    println!("");
    println!("Day 3:");
    day3::day3::diagnostic_power_consumption();
    day3::day3::diagnostic_life_support();
}
