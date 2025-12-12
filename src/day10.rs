use std::collections::{HashMap, HashSet, VecDeque};

use regex::Regex;

use crate::utils::read_file_contents;

fn generate_patterns(buttons: &[u32]) -> HashMap<Vec<u32>, u32> {
    let mut out = HashMap::new();
    let n = buttons.len();

    for i in 0u64..(1 << n) {
        let mut pattern = vec![0u32; 32];
        let mut cost = 0;

        for (idx, &b) in buttons.iter().enumerate() {
            if (i >> idx) & 1 == 1 {
                cost += 1;
                for bit in 0..32 {
                    if (b >> bit) & 1 == 1 {
                        pattern[bit] += 1;
                    }
                }
            }
        }

        out.entry(pattern)
            .and_modify(|e: &mut u32| *e = (*e).min(cost))
            .or_insert(cost);
    }
    out
}

fn solve_aux(
    goal: Vec<u32>,
    patterns: &HashMap<Vec<u32>, u32>,
    memo: &mut HashMap<Vec<u32>, u32>,
) -> u32 {
    if goal.iter().all(|&x| x == 0) {
        return 0;
    }

    if let Some(&res) = memo.get(&goal) {
        return res;
    }

    let mut best = u32::MAX;

    for (pattern, &cost) in patterns {
        let mut possible = true;
        for (p, g) in pattern.iter().zip(goal.iter()) {
            if !(*p <= *g && p % 2 == g % 2) {
                possible = false;
                break;
            }
        }

        if possible {
            let new_goal: Vec<u32> = pattern
                .iter()
                .zip(goal.iter())
                .map(|(p, g)| (g - p) / 2)
                .collect();

            let res = solve_aux(new_goal, patterns, memo);
            if res != u32::MAX {
                if let Some(doubled) = res.checked_mul(2) {
                    if let Some(total) = cost.checked_add(doubled) {
                        best = best.min(total);
                    }
                }
            }
        }
    }

    memo.insert(goal.clone(), best);
    best
}

fn solve_single(goal: Vec<u32>, buttons: Vec<u32>) -> Option<u32> {
    let patterns = generate_patterns(&buttons);
    let mut memo = HashMap::new();

    let res = solve_aux(goal, &patterns, &mut memo);
    if res == u32::MAX {
        None
    } else {
        Some(res)
    }
}

fn check_line(indicator: u32, buttons: Vec<u32>) -> Option<u32> {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));

    while let Some((state, count)) = queue.pop_front() {
        if seen.contains(&state) {
            continue;
        }
        seen.insert(state);

        if state == indicator {
            return Some(count);
        }

        let nc = count + 1;
        for b in &buttons {
            queue.push_back((state ^ *b, nc));
        }
    }

    return None;
}

pub fn run(second: bool) {
    let content = read_file_contents("day10.txt");

    let regex = Regex::new(r"\[([.#]+)]|\((\d(,\d)*)\)|\{(\d+(,\d+)*)}").unwrap();

    let lines = content
        .lines()
        .map(|line| {
            let captures = regex
                .captures_iter(line)
                .flat_map(|caps| {
                    caps.iter()
                        .skip(1)
                        .flatten()
                        .map(|m| m.as_str().to_string())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            let mut captures = captures.into_iter();

            let ind = captures.next().unwrap().replace('#', "1").replace('.', "0");
            let ind: String = ind.chars().rev().collect();
            let indicator = u32::from_str_radix(&ind, 2).unwrap();

            let mut buttons = captures
                .filter(|line| !line.starts_with(','))
                .collect::<Vec<_>>();

            let goal = buttons.remove(buttons.len() - 1);

            let buttons = buttons
                .into_iter()
                .map(|button| {
                    let mut mask = 0;

                    for n in button.split(',') {
                        mask |= 1u32 << &u32::from_str_radix(&n, 10).unwrap();
                    }

                    mask
                })
                .collect();

            // parse goal
            let split_goal = goal
                .split(',')
                .map(|n| u32::from_str_radix(n, 10).unwrap())
                .collect::<Vec<u32>>();

            (indicator, buttons, split_goal)
        })
        .collect::<Vec<_>>();

    if !second {
        phase1(lines);
    } else {
        phase2(lines);
    }
}

pub fn phase2(lines: Vec<(u32, Vec<u32>, Vec<u32>)>) {
    let total = lines
        .into_iter()
        .flat_map(|(_, buttons, goal)| solve_single(goal, buttons))
        .sum::<u32>();

    println!("ANS: {:?}", total);
}

pub fn phase1(lines: Vec<(u32, Vec<u32>, Vec<u32>)>) {
    let total = lines
        .into_iter()
        .flat_map(|(indicator, buttons, _)| check_line(indicator, buttons))
        .sum::<u32>();

    println!("ANS: {:?}", total);
}
