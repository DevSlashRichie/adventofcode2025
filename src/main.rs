mod day1;
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
        _ => println!("[ERROR] day not found."),
    }
}
