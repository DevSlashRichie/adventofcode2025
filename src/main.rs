mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod utils;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    create: bool,
    #[arg(short, long)]
    day: u8,
    #[arg(short, long)]
    second: bool,
}

fn main() {
    let args = Args::parse();

    match args.day {
        1 => day1::run(args.second),
        2 => day2::run(args.second),
        3 => day3::run(args.second),
        4 => day4::run(args.second),
        5 => day5::run(args.second),
        6 => day6::run(args.second),
        7 => day7::run(args.second),
        8 => day8::run(args.second),
        9 => day9::run(args.second),
        10 => day10::run(args.second),
        11 => day11::run(args.second),
        12 => day12::run(args.second),
        _ => println!("[ERROR] day not found."),
    }
}
