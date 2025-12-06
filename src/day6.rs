use itertools::Itertools;
use regex::Regex;

use crate::utils::{read_file_as_lines, read_file_contents};

pub fn phase1() {
    let re = Regex::new(r"([0-9]+)").unwrap();

    let contents = read_file_as_lines("day6.txt");

    let operations = contents[contents.len() - 2]
        .chars()
        .filter(|line| !line.is_whitespace())
        .collect::<Vec<_>>();

    let contents = contents
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            re.captures_iter(&line)
                .map(|capture| {
                    let (_, [number]) = capture.extract();
                    u64::from_str_radix(number, 10).unwrap()
                })
                .collect::<Vec<_>>()
        })
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let cols = contents[0].len();

    let ans: u64 = (0..cols)
        .map(|index| {
            let nums = contents.iter().map(|contents| contents[index]);

            let ans: u64 = if operations[index] == '+' {
                nums.sum()
            } else {
                nums.product()
            };

            ans
        })
        .sum();

    println!("ANS: {ans}");
}

pub fn run(second: bool) {
    if !second {
        phase1();
    }
}
