use regex::Regex;

use crate::utils::read_file_as_lines;

pub fn phase2() {
    let contents = read_file_as_lines("day6.txt");

    let cols = contents[0].len();

    let mut last_sim = ' ';
    let mut total_ans: u64 = 0;
    let mut col_acc = Vec::new();
    for index in 0..cols {
        let mut acc = 0;
        let mut mult = 1;
        for line in contents.iter().rev().filter(|f| !f.is_empty()) {
            let line = line.chars().collect::<Vec<_>>();

            if line[index] == '*' || line[index] == '+' {
                last_sim = line[index];
            } else {
                if let Some(num) = line[index].to_digit(10) {
                    acc += num as u64 * mult;
                    mult *= 10;
                }
            }
        }

        if acc > 0 {
            col_acc.push(acc);
        } else {
            let ans = if last_sim == '*' {
                col_acc.iter().product::<u64>()
            } else if last_sim == '+' {
                col_acc.iter().sum::<u64>()
            } else {
                0
            };
            total_ans += ans;

            col_acc.clear();
        }
    }

    if last_sim == '*' {
        total_ans += col_acc.iter().product::<u64>();
    } else if last_sim == '+' {
        total_ans += col_acc.iter().sum::<u64>();
    }

    println!("ANS: {total_ans}");
}

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
    } else {
        phase2();
    }
}
