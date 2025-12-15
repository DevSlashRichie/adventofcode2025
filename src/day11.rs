use std::collections::{HashMap, HashSet, VecDeque};

use rustc_hash::FxHashSet;

use crate::utils::read_file_contents;

pub fn phase1(devices_and_outputs: HashMap<String, Vec<String>>) {
    let mut q = VecDeque::new();

    let starting_points = devices_and_outputs.get("you").unwrap();
    for point in starting_points {
        q.push_back(point.clone());
    }

    let mut ans = 0;
    while let Some(next) = q.pop_front() {
        if let Some(next_points) = devices_and_outputs.get(&next) {
            for point in next_points {
                if point == "out" {
                    ans += 1;
                    break;
                }

                q.push_back(point.clone());
            }
        }
    }

    println!("ANS: {}", ans);
}

// for this solution I asked gemini as it was out of my league.

const MASK_NONE: u8 = 0;
const MASK_DAC: u8 = 1;
const MASK_FFT: u8 = 2;
const MASK_BOTH: u8 = 3;

pub fn phase2(devices_and_outputs: HashMap<String, Vec<String>>) {
    print_all_paths(devices_and_outputs.clone());

    // Memoization Cache: Key = (NodeName, CurrentState), Value = PathCount
    let mut memo: HashMap<(String, u8), u64> = HashMap::new();

    // Helper function for DP
    fn count_paths(
        curr: &str,
        state: u8,
        graph: &HashMap<String, Vec<String>>,
        memo: &mut HashMap<(String, u8), u64>,
    ) -> u64 {
        // 1. Base Case: Reached Output
        if curr == "out" {
            return if state == MASK_BOTH { 1 } else { 0 };
        }

        // 2. Check Cache (The Speedup)
        // If we have solved this exact problem (node + state) before, return the answer.
        let key = (curr.to_string(), state);
        if let Some(&count) = memo.get(&key) {
            return count;
        }

        // 3. Recursive Step
        let mut total_paths = 0;
        if let Some(neighbors) = graph.get(curr) {
            for next_node in neighbors {
                // Determine new state based on the next node
                let mut next_state = state;
                if next_node == "dac" {
                    next_state |= MASK_DAC;
                }
                if next_node == "fft" {
                    next_state |= MASK_FFT;
                }

                total_paths += count_paths(next_node, next_state, graph, memo);
            }
        }

        // 4. Store Result in Cache
        memo.insert(key, total_paths);
        total_paths
    }

    // Start execution
    let mut ans = 0;
    if let Some(starts) = devices_and_outputs.get("svr") {
        for start_point in starts {
            // Determine initial state for the starting nodes
            let mut init_state = MASK_NONE;
            if start_point == "dac" {
                init_state |= MASK_DAC;
            }
            if start_point == "fft" {
                init_state |= MASK_FFT;
            }

            ans += count_paths(start_point, init_state, &devices_and_outputs, &mut memo);
        }
    }

    println!("ANS: {}", ans);
}

pub fn print_all_paths(devices_and_outputs: HashMap<String, Vec<String>>) {
    let mut ans = 0;

    // Recursive DFS function
    // current_path: Keeps the list of nodes visited so far (for printing)
    // visited_set: Keeps the set of nodes for fast cycle detection
    fn dfs(
        current: &str,
        graph: &HashMap<String, Vec<String>>,
        current_path: &mut Vec<String>,
        visited_set: &mut HashSet<String>,
        found_dac: bool,
        found_fft: bool,
        count: &mut usize,
    ) {
        // 1. Base Case: Reached "out"
        if current == "out" {
            if found_dac && found_fft {
                *count += 1;
                // PRINT THE PATH HERE
            }
            println!("Found Path #{}: {:?}", *count, current_path);
            return;
        }

        // 2. Cycle Detection
        // If 'current' is already in our set, we are in a loop. Stop.
        if !visited_set.insert(current.to_string()) {
            return;
        }

        // 3. Update State
        let has_dac = found_dac || current == "dac";
        let has_fft = found_fft || current == "fft";

        // 4. Explore Neighbors
        if let Some(neighbors) = graph.get(current) {
            for neighbor in neighbors {
                // Add neighbor to the path for display
                current_path.push(neighbor.clone());

                // Recurse
                dfs(
                    neighbor,
                    graph,
                    current_path,
                    visited_set,
                    has_dac,
                    has_fft,
                    count,
                );

                // Backtrack: remove neighbor from path to try next neighbor
                current_path.pop();
            }
        }

        // 5. Backtrack: remove 'current' from set so other paths can use it
        visited_set.remove(current);
    }

    // --- Start Execution ---
    let mut visited_set = HashSet::new();

    // We start the path with "svr"
    // Note: The logic assumes "svr" itself is not part of the repeated graph structure
    if let Some(starts) = devices_and_outputs.get("svr") {
        for start_point in starts {
            let mut path = vec!["svr".to_string(), start_point.clone()];
            visited_set.insert("svr".to_string()); // Mark svr as visited

            dfs(
                start_point,
                &devices_and_outputs,
                &mut path,
                &mut visited_set,
                false,
                false,
                &mut ans,
            );

            visited_set.remove("svr"); // Clean up
        }
    }

    println!("Total Paths Found: {}", ans);
}

pub fn run(s: bool) {
    let content = read_file_contents("day11.txt");

    let devices_and_outputs = content
        .lines()
        .map(|line| {
            let (device, outputs) = line.split_once(':').unwrap();
            let outputs = outputs
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();

            (device.to_string(), outputs)
        })
        .collect::<HashMap<_, _>>();

    if !s {
        phase1(devices_and_outputs);
    } else {
        phase2(devices_and_outputs);
    }
}
