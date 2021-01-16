use std::env;
use std::path::Path;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args.get(1).map(String::as_str);

    let success = match day {
        Some("01") => day01::run(&Path::new("input/day01.txt")),
        Some("02") => day02::run(&Path::new("input/day02.txt")),
        Some("03") => day03::run(&Path::new("input/day03.txt")),
        Some("04") => day04::run(&Path::new("input/day04.txt")),
        Some("05") => day05::run(&Path::new("input/day05.txt")),
        _ => { eprintln!("day not found"); false }
    };
    
    if !success { std::process::exit(1); }
}
