use std::collections::{HashMap, HashSet};

use crate::utils::read_file_contents;
use itertools::Itertools;
use min_heap::MinHeap;
use rustc_hash::FxHashMap;

pub fn sig(a: (i64, i64, i64)) -> i64 {
    a.0.pow(2) + a.1.pow(2) + a.2.pow(3)
}

pub fn euclidean_distance(a: (i64, i64, i64), b: (i64, i64, i64)) -> usize {
    let n = (b.0 - a.0).pow(2) + (b.1 - a.1).pow(2) + (b.2 - a.2).pow(2);

    (n as f64).sqrt() as usize
}

pub fn run(second: bool) {
    let contents = read_file_contents("day8.txt");
    let numbers = contents
        .lines()
        .map(|f| {
            let mut numbers = f
                .split(',')
                .map(|num| i64::from_str_radix(num, 10).unwrap());

            (
                numbers.next().unwrap(),
                numbers.next().unwrap(),
                numbers.next().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut links = FxHashMap::<(i64, i64, i64), i64>::default();
    let mut circuits = FxHashMap::<i64, Vec<(i64, i64, i64)>>::default();

    let mut heap = numbers
        .iter()
        .flat_map(|coord| {
            let distances = numbers
                .iter()
                .filter(|other| *other != coord && coord < *other)
                .map(|other| (euclidean_distance(*coord, *other), *coord, *other))
                .collect::<Vec<_>>();

            distances
        })
        .collect::<MinHeap<_>>();

    println!("{}", heap.len());

    for _ in 0..1000 {
        if let Some((_, a, b)) = heap.pop() {
            let a_circuit = *links.get(&a).unwrap_or(&sig(a));
            let b_circuit = *links.get(&b).unwrap_or(&sig(b));

            if a_circuit == b_circuit {
                continue;
            }

            let a_childs = circuits.get(&a_circuit).cloned().unwrap_or(vec![a]);
            let b_childs = circuits.get(&b_circuit).cloned().unwrap_or(vec![b]);

            let choosen_id = if a_childs.len() < b_childs.len() {
                b_circuit
            } else {
                a_circuit
            };

            let all_childs = a_childs
                .iter()
                .chain(b_childs.iter())
                .collect::<HashSet<_>>();

            for coord in &all_childs {
                links.insert(**coord, choosen_id);
            }

            circuits.insert(choosen_id, all_childs.into_iter().copied().collect());
        }
    }

    let ans = circuits
        .into_values()
        .map(|f| f.len())
        .sorted_unstable_by(|a, b| b.cmp(a))
        .take(3)
        .product::<usize>();

    println!("ANS: {}", ans);
}
