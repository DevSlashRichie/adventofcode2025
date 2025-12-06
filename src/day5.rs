use std::collections::HashSet;
use std::ops::RangeInclusive;

use crate::utils::read_file_contents;

fn overlaps<T>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> bool
where
    T: Ord + Copy,
{
    let (a_start, a_end) = (*a.start(), *a.end());
    let (b_start, b_end) = (*b.start(), *b.end());

    a_start <= b_end && b_start <= a_end
}

fn merge_ranges<T>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> RangeInclusive<T>
where
    T: Ord + Copy,
{
    let lowest_from = *a.start().min(b.start());
    let highest_to = *a.end().max(b.end());

    lowest_from..=highest_to
}

fn parse_number(s: &str) -> u64 {
    s.parse().expect("invalid number")
}

pub fn run(second: bool) {
    let contents = read_file_contents("day5.txt").trim().to_string();
    let lines: Vec<&str> = contents.lines().collect();

    let ranges = lines
        .iter()
        .take_while(|line| !line.is_empty())
        .map(|f| f.to_string())
        .collect::<Vec<_>>();

    let ranges = ranges
        .into_iter()
        .map(|line| {
            let (from, to) = line.split_once('-').expect("invalid range");
            let (from, to) = (parse_number(from), parse_number(to));

            from..=to
        })
        .collect::<Vec<_>>();

    if !second {
        let available = lines
            .iter()
            .skip(ranges.len() + 1)
            .map(|line| line.to_string())
            .collect::<Vec<_>>();

        let ingredients = available
            .into_iter()
            .map(|line| parse_number(&line))
            .collect::<HashSet<_>>();

        let available = ingredients
            .into_iter()
            .filter(|ing| ranges.iter().any(|range| range.contains(ing)))
            .count();

        println!("ANS: {available}");
    } else {
        //let ans: usize = ranges.into_iter().map(|range| range.count()).sum();
        //println!("ANS: {ans}");

        let mut ranges = ranges;
        ranges.sort_by_key(|l| *l.start());

        println!("{:#?}", ranges);

        let mut index = 1;

        while index < ranges.len() {
            let prev = &ranges[index - 1];
            let curr = &ranges[index];

            if overlaps(prev, curr) {
                let n = merge_ranges(prev, curr);
                ranges[index] = n;
                ranges.remove(index - 1);
            } else {
                index += 1;
            }
        }

        println!("{:#?}", ranges);

        let total: usize = ranges.into_iter().map(|range| range.count()).sum();

        println!("ANS: {total}");
    }
}
