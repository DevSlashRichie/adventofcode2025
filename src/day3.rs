use std::{cmp::Reverse, collections::BinaryHeap};

use crate::utils::read_file_as_lines;

fn explore_line(line: String) -> u32 {
    let line = line
        .chars()
        .map(|f| u32::from_str_radix(&f.to_string(), 10).unwrap())
        .collect::<Vec<_>>();

    let mut l = 0;
    let mut r = 1;

    let mut max_l = 0;
    let mut max_r = 0;

    while l <= r && r < line.len() {
        max_r = max_r.max(line[r]);
        max_l = max_l.max(line[l]);

        if line[r] > line[l] && r != line.len() - 1 {
            l = r;

            // we need to reset r, because to join the end nubmer should depend
            // on order of appearance.
            max_r = 0;
        }

        r += 1;
    }

    max_l * 10 + max_r
}

fn explore_line_v2(line: String) -> u64 {
    let line = line
        .chars()
        .map(|f| u32::from_str_radix(&f.to_string(), 10).unwrap())
        .collect::<Vec<_>>();

    let mut l = 0;
    let mut r = 0;

    let mut acc = Vec::<u32>::with_capacity(12);

    while l <= r && r < line.len() {
        let acs = acc
            .iter()
            .copied()
            .map(|f| f.to_string())
            .collect::<String>();

        //print!("{}", acs);
        while acc.len() > 0 && acc[acc.len() - 1] < line[r] && ((line.len() - r) > (12 - acc.len()))
        {
            let r = acc.pop().unwrap();
            //print!("[{r}]");
        }

        //println!();

        if acc.len() < 12 {
            acc.push(line[r]);
            //println!("AP{r}: {}", line[r]);
        }

        if line[r] > line[l] && r != line.len() - 1 {
            // TODO
        }

        r += 1;
    }

    //println!();

    acc.into_iter()
        .map(|num| num.to_string())
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

pub fn _run(second: bool) {
    let r = explore_line_v2(
        "
818181911112111
"
        .trim()
        .to_owned(),
    );
    println!("{r}");
}

pub fn run(second: bool) {
    let total_sum: u64 = read_file_as_lines("day3.txt")
        .into_iter()
        .map(|f| f.trim().to_owned())
        .filter(|f| !f.is_empty())
        .map(|line| {
            if second {
                explore_line_v2(line)
            } else {
                explore_line(line) as u64
            }
        })
        .sum();

    println!("{:#?}", total_sum);
}
