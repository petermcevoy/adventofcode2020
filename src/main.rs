use std::env;
use std::path::Path;

mod day01;
mod day02;
mod day03;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args.get(1).map(String::as_str);

    let success = match day {
        Some("01") => day01::run(&Path::new("input/day01.txt")),
        Some("02") => day02::run(&Path::new("input/day02.txt")),
        Some("03") => day03::run(&Path::new("input/day03.txt")),
        _ => { eprintln!("day not found"); false }
    };
    
    if !success { std::process::exit(1); }
}
