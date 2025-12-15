// thanks to memes
use crate::utils::read_file_contents;

#[derive(Debug)]
struct Case {
    w: u32,
    l: u32,
    counts: Vec<u32>,
}

pub fn run(second: bool) {
    let content = read_file_contents("day12.txt");

    let s = content.split("\n\n").collect::<Vec<_>>();

    let shapes = s.iter().take(s.len() - 1).map(|shape| {
        let (id, figure) = shape.split_once(':').unwrap();

        //let id: u32 = id.parse().unwrap();
    });

    let cases = &s[s.len() - 1];
    let cases = cases
        .lines()
        .map(|case| {
            let (size, shape_counts) = case.split_once(':').unwrap();

            let (w, l) = size.split_once('x').unwrap();

            let counts = shape_counts
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            Case {
                w: w.parse().unwrap(),
                l: l.parse().unwrap(),
                counts,
            }
        })
        .collect::<Vec<_>>();
    let count = cases
        .into_iter()
        .map(|case| {
            let total = case.counts.iter().sum();

            (case.w / 3) * (case.l / 3) >= total
        })
        .filter(|case| *case)
        .count();

    println!("ANS: {}", count);
}
