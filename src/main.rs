use std::env;

mod day01;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];

    let mut pass_args = args.clone();
    pass_args.remove(1);

    match &day[..] {
        "01" => day01::run(pass_args),
        _ => println!("day not found")
    }
}
