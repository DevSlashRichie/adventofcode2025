use std::os::unix::fs::OpenOptionsExt;

use crate::utils::read_file_contents;
use itertools::Itertools;
use min_heap::MinHeap;
use rustc_hash::FxHashMap;

type Coord = (i64, i64, i64);

pub fn euclidean_distance(a: Coord, b: Coord) -> usize {
    let n = (b.0 - a.0).pow(2) + (b.1 - a.1).pow(2) + (b.2 - a.2).pow(2);

    (n as f64).sqrt() as usize
}

pub fn phase1(numbers: Vec<Coord>, n: usize) {
    let mut heap = numbers
        .iter()
        .flat_map(|coord| {
            let distances = numbers
                .iter()
                .filter(|other| other != &coord && coord < other)
                .map(|other| (euclidean_distance(*coord, *other), coord, other))
                .collect::<Vec<_>>();

            distances
        })
        .collect::<MinHeap<_>>();

    let mut parents = FxHashMap::default();
    let mut size = FxHashMap::default();

    for num in &numbers {
        size.insert(*num, 1);
        parents.insert(*num, *num);
    }

    fn find(coord: Coord, parents: &mut FxHashMap<Coord, Coord>) -> Coord {
        if let Some(found) = parents.get(&coord) {
            if &coord == found {
                return coord;
            }

            let f = find(*found, parents);
            parents.insert(coord, f);

            return f;
        }

        (0, 0, 0)
    }

    for _ in 0..n {
        if let Some((_, a, b)) = heap.pop() {
            let root_a = find(*a, &mut parents);
            let root_b = find(*b, &mut parents);

            if root_a == root_b {
                continue;
            }

            let (p1, p2) = match size.get(&root_a).zip(size.get(&root_b)) {
                Some((a, b)) => {
                    if a < b {
                        (root_a, root_b)
                    } else {
                        (root_b, root_a)
                    }
                }
                None => continue,
            };

            if let Some(p1_size) = size.get(&p1).copied() {
                size.entry(p2).and_modify(|value| *value += p1_size);
                parents.insert(p1, p2);
            }
        }
    }

    let ans = parents
        .iter()
        .filter(|(a, b)| a == b)
        .map(|(entry, _)| size.get(entry).copied().unwrap())
        .sorted_unstable_by(|a, b| b.cmp(a))
        .take(3)
        .product::<i64>();

    println!("ANS: {:#?}", ans);
}

pub fn phase2(numbers: Vec<Coord>) {
    let mut heap = numbers
        .iter()
        .flat_map(|coord| {
            let distances = numbers
                .iter()
                .filter(|other| other != &coord && coord < other)
                .map(|other| (euclidean_distance(*coord, *other), coord, other))
                .collect::<Vec<_>>();

            distances
        })
        .collect::<MinHeap<_>>();

    let mut parents = FxHashMap::default();
    let mut size = FxHashMap::default();

    for num in &numbers {
        size.insert(*num, 1);
        parents.insert(*num, *num);
    }

    fn find(coord: Coord, parents: &mut FxHashMap<Coord, Coord>) -> Option<Coord> {
        if let Some(found) = parents.get(&coord) {
            if &coord == found {
                return Some(coord);
            }

            let f = find(*found, parents);
            if let Some(f) = f {
                parents.insert(coord, f);
            }

            return f;
        }

        None
    }

    while let Some((_, a, b)) = heap.pop() {
        let (root_a, root_b) = match find(*a, &mut parents).zip(find(*b, &mut parents)) {
            Some((a, b)) => (a, b),
            None => continue,
        };

        if root_a == root_b {
            continue;
        }

        let (p1, p2) = match size.get(&root_a).zip(size.get(&root_b)) {
            Some((a, b)) => {
                if a < b {
                    (root_a, root_b)
                } else {
                    (root_b, root_a)
                }
            }
            None => continue,
        };

        if let Some(p1_size) = size.get(&p1).copied() {
            size.entry(p2).and_modify(|value| *value += p1_size);
            parents.insert(p1, p2);
            size.remove(&p1);
        }

        if size.len() == 1 {
            let ans = a.0 * b.0;
            println!("ANS: {ans}");
            break;
        }
    }

    //println!("PHASE 2 ANS: {:?}", size);
}

pub fn run(second: bool) {
    let contents = read_file_contents("day8.txt");
    let numbers = contents
        .lines()
        .map(|coord| {
            let mut numbers = coord
                .split(',')
                .map(|num| i64::from_str_radix(num, 10).unwrap());

            (
                numbers.next().unwrap(),
                numbers.next().unwrap(),
                numbers.next().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    if !second {
        // use 10 for example, use 1000 for problem.
        phase1(numbers, 10);
    } else {
        phase2(numbers);
    }
}
