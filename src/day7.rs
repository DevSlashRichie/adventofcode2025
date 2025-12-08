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

pub fn run(second: bool) {
    if second {
        phase2();
    } else {
        phase1();
    }
}
