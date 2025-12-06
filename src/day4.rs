use std::collections::HashSet;

use crate::utils::{print_matrix, read_file_as_lines};

const DIRS: [(isize, isize); 8] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

fn count_adjacent(matrix: &Vec<Vec<i32>>, (row, col): (usize, usize)) -> usize {
    let cols = matrix.len();
    let rows = matrix[0].len();

    DIRS.iter()
        .copied()
        .filter(|(dc, dr)| {
            if let (Some(nc), Some(nr)) = (col.checked_add_signed(*dc), row.checked_add_signed(*dr))
            {
                nc < cols && nr < rows && matrix[nr][nc] > 0
            } else {
                false
            }
        })
        .count()
}

pub fn phase1(matrix: &Vec<Vec<i32>>) -> usize {
    // just because I want to replace each to 2
    let mut matrix = matrix.clone();

    let cols = matrix.len();
    let rows = matrix[0].len();

    let mut ans = 0;
    for row in 0..rows {
        for col in 0..cols {
            if matrix[row][col] == 0 {
                continue;
            }

            let count = count_adjacent(&matrix, (row, col));

            if count < 4 {
                matrix[row][col] = 2;

                ans += 1;
            }
        }
    }

    ans
}

// (current, total)
fn remove(matrix: &mut Vec<Vec<i32>>, acc: usize) -> (usize, usize) {
    let cols = matrix.len();
    let rows = matrix[0].len();

    let mut ans = 0;
    for row in 0..rows {
        for col in 0..cols {
            if matrix[row][col] == 0 {
                continue;
            }

            let count = count_adjacent(&matrix, (row, col));

            if count < 4 {
                matrix[row][col] = 0;

                ans += 1;
            }
        }
    }

    if ans == 0 {
        (ans, acc + ans)
    } else {
        remove(matrix, acc + ans)
    }
}

fn phase2(matrix: &Vec<Vec<i32>>) -> usize {
    // just because I want to replace each to 2
    let mut matrix = matrix.clone();

    let (_, ans) = remove(&mut matrix, 0);

    ans
}

pub fn run(second: bool) {
    let rows = read_file_as_lines("day4.txt");

    // create a matrix.
    let matrix = rows
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.trim()
                .chars()
                .map(|ch| match ch {
                    '.' => 0,
                    '@' => 1,
                    _ => panic!("invalid char"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    if second {
        let ans_2 = phase2(&matrix);
        println!("ANS: {ans_2}");
    } else {
        let ans_1 = phase1(&matrix);
        println!("ANS: {ans_1}");
    }
}
