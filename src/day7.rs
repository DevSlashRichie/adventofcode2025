use std::collections::VecDeque;

use rustc_hash::FxHashMap;

use crate::utils::read_file_contents;

fn next_but_sides(y: usize, x: usize) -> [(usize, usize); 2] {
    [(y, x + 1), (y, x - 1)]
}

fn phase1() {
    let contents = read_file_contents("day7.txt");

    let mut matrix = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let rows = matrix.len();
    let cols = matrix[0].len();

    let entry_point = (1_usize, cols / 2);

    let mut q = VecDeque::new();
    q.push_back((entry_point, 0));

    //print_matrix(&matrix);

    let mut ans = 0;
    while !q.is_empty() {
        let ((y, x), count) = q.pop_back().unwrap();

        if matrix[y][x] != '|' && matrix[y][x] != '^' {
            matrix[y][x] = '|';
        }

        if matrix[y][x] == '^' {
            ans += 1;
            for (sy, sx) in next_but_sides(y, x) {
                if matrix[sy][sx] != '|' {
                    q.push_back(((sy, sx), count + 1));
                }
            }
        } else if y + 1 < rows && x < cols {
            let (ny, nx) = (y + 1, x);

            if matrix[ny][nx] == '^' {
                q.push_front(((ny, nx), count));
            } else if matrix[ny][nx] == '.' {
                q.push_back(((ny, nx), count));
            }
        }
    }

    println!("ANS: {ans}");
}

fn phase2() {
    let contents = read_file_contents("day7.txt");

    let matrix = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let cols = matrix[0].len();

    let mut cache = FxHashMap::default();

    let entry_point = (1_usize, cols / 2);

    #[inline]
    fn dfs(
        (y, x): (usize, usize),
        matrix: &Vec<Vec<char>>,
        cache: &mut FxHashMap<(usize, usize), usize>,
    ) -> usize {
        if let Some(count) = cache.get(&(y, x)) {
            return *count;
        }

        if y + 1 >= matrix.len() {
            cache.insert((y, x), 1);
            return 1;
        }

        if matrix[y][x] == '^' {
            let ans = dfs((y, x + 1), matrix, cache) + dfs((y, x - 1), matrix, cache);

            cache.insert((y, x), ans);

            ans
        } else {
            dfs((y + 1, x), matrix, cache)
        }
    }

    let ans = dfs(entry_point, &matrix, &mut cache);
    println!("ANS: {ans}");
}

fn phase2_norec() {
    use rustc_hash::FxHashMap;

    let contents = read_file_contents("day7.txt");

    let matrix = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let rows = matrix.len();
    let cols = matrix[0].len();

    let entry_point = (1usize, cols / 2);

    let mut cache: FxHashMap<(usize, usize), usize> = FxHashMap::default();

    // Stack frame: (y, x, state)
    // state = 0 → first time visiting; push children
    // state = 1 → children resolved; compute value
    let mut stack: Vec<((usize, usize), u8)> = Vec::new();
    stack.push((entry_point, 0));

    while let Some(((y, x), state)) = stack.pop() {
        // Memoized?
        if let Some(_) = cache.get(&(y, x)) {
            continue;
        }

        if state == 0 {
            // Base case: bottom row
            if y + 1 >= rows {
                cache.insert((y, x), 1);
                continue;
            }

            // Need to resolve children first
            // Push the node again with state=1 (post-order)
            stack.push(((y, x), 1));

            if matrix[y][x] == '^' {
                // Branch: right, left
                stack.push(((y, x + 1), 0));
                stack.push(((y, x - 1), 0));
            } else {
                // Straight down
                stack.push(((y + 1, x), 0));
            }
        } else {
            // state == 1 → children resolved; we can compute

            let val = if matrix[y][x] == '^' {
                let right = *cache.get(&(y, x + 1)).unwrap();
                let left = *cache.get(&(y, x - 1)).unwrap();
                right + left
            } else {
                *cache.get(&(y + 1, x)).unwrap()
            };

            cache.insert((y, x), val);
        }
    }

    let ans = cache[&entry_point];
    println!("ANS: {ans}");
}

fn phase2_dp_bottom_up() {
    let contents = read_file_contents("day7.txt");

    let matrix = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let rows = matrix.len();
    let cols = matrix[0].len();

    // first point: S
    let (iy, ix) = (1usize, cols / 2);

    // Initialize DP table: DP[y][x] stores the number of paths from (y, x) to the end.
    // Start with all zeros.
    let mut dp: Vec<Vec<usize>> = vec![vec![0; cols]; rows];

    // BASE CASE
    for x in 0..cols {
        dp[rows - 1][x] = 1;
    }

    for y in (iy..rows - 1).rev() {
        for x in 0..cols {
            if matrix[y][x] == '^' {
                // Branching down and sideways: (y+1, x-1) and (y+1, x+1)
                // you need to add up at Y because otherwise you need to find a way
                // to process the sides of the ^.
                dp[y][x] = dp[y + 1][x - 1] + dp[y + 1][x + 1];
            } else {
                // Straight down: (y+1, x)
                dp[y][x] = dp[y + 1][x];
            }
        }
    }

    let ans = dp[iy][ix];
    println!("ANS: {ans}");
}

pub fn run(second: bool) {
    if second {
        phase2_norec();
        phase2();
        phase2_dp_bottom_up();
    } else {
        phase1();
    }
}
