use std::{
    any::Any,
    collections::{HashSet, VecDeque},
    fmt::Display,
    str::FromStr,
    time::Instant,
};

use itertools::Itertools;

use crate::utils::{print_matrix, read_file_contents};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Coord {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("({}, {})", self.x, self.y))
    }
}

impl Coord {
    pub fn area(&self, rhs: &Self) -> usize {
        let w = self.x.abs_diff(rhs.x) + 1;
        let h = self.y.abs_diff(rhs.y) + 1;

        w * h
    }

    pub fn neighbors(&self) -> Vec<Self> {
        vec![
            Self {
                x: self.x + 1,
                y: self.y,
            },
            Self {
                x: self.x - 1,
                y: self.y,
            },
            Self {
                x: self.x,
                y: self.y + 1,
            },
            Self {
                x: self.x,
                y: self.y - 1,
            },
        ]
    }
}

impl FromStr for Coord {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or("invalid coord".to_string())?;

        Ok(Self {
            x: x.parse::<usize>().map_err(|err| err.to_string())?,
            y: y.parse::<usize>().map_err(|err| err.to_string())?,
        })
    }
}

pub fn run(second: bool) {
    let content = read_file_contents("day9.txt");

    if !second {
        let now = Instant::now();
        phase1(content);
        println!("Took: {:?}", now.elapsed());
    } else {
        let now = Instant::now();
        phase2(content);
        //_phase2_discarded(content);
        println!("Took: {:?}", now.elapsed());
    }
}

fn phase1(content: String) {
    let tiles = content
        .lines()
        .map(|f| Coord::from_str(f).expect("invalid coord"))
        .collect::<Vec<_>>();

    let all = tiles
        .iter()
        .flat_map(|coord| {
            tiles
                .iter()
                .filter(|other| other != &coord && other < &coord)
                .map(|other| coord.area(other))
                .collect::<Vec<_>>()
        })
        .max()
        .unwrap();

    println!("ANS: {}", all);
}

pub fn create_matrix(rows: usize, cols: usize) -> Vec<Vec<char>> {
    vec![vec!['.'; cols]; rows]
}

// thanks to icub3d
fn phase2(content: String) {
    let tiles = content
        .lines()
        .map(|f| Coord::from_str(f).expect("invalid coord"))
        .collect::<Vec<_>>();

    let edges = tiles
        .iter()
        .copied()
        .tuple_windows()
        .collect::<Vec<(Coord, Coord)>>()
        .into_iter()
        .chain([(
            tiles.last().copied().unwrap(),
            tiles.first().copied().unwrap(),
        )])
        .collect::<Vec<_>>();

    let all = tiles
        .iter()
        .flat_map(|coord| {
            tiles
                .iter()
                .filter(|other| other != &coord && other < &coord)
                .map(|other| (*coord, *other, coord.area(other)))
                .collect::<Vec<_>>()
        })
        .sorted_by_key(|a| a.2)
        .rev()
        .find(|(a, b, _)| {
            // make sure it's inside

            let ans = edges.iter().all(|(start, end)| {
                let before = a.x.max(b.x) <= start.x.min(end.x);
                let after = a.x.min(b.x) >= start.x.max(end.x);
                let above = a.y.max(b.y) <= start.y.min(end.y);
                let below = a.y.min(b.y) >= start.y.max(end.y);

                before || after || above || below
            });

            ans
        })
        .unwrap()
        .2;

    println!("ANS: {all}");
}

fn flood_fill(grid: &mut Vec<Vec<char>>, x: usize, y: usize, target: &[char], replacement: char) {
    let rows = grid.len();
    let cols = grid[0].len();

    if !target.contains(&grid[y][x]) {
        return;
    }

    let mut queue = std::collections::VecDeque::new();
    queue.push_back((x, y));
    grid[y][x] = replacement;

    while let Some((cx, cy)) = queue.pop_front() {
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let nx = cx as isize + dx;
            let ny = cy as isize + dy;

            if nx >= 0
                && ny >= 0
                && (nx as usize) < cols
                && (ny as usize) < rows
                //&& grid[ny as usize][nx as usize] == target
                && target.contains(&grid[ny as usize][nx as usize])
            {
                grid[ny as usize][nx as usize] = replacement;
                queue.push_back((nx as usize, ny as usize));
            }
        }
    }
}

fn rect_is_all_I(a: Coord, b: Coord, matrix: &Vec<Vec<char>>) -> bool {
    let (min_x, max_x) = if a.x < b.x { (a.x, b.x) } else { (b.x, a.x) };
    let (min_y, max_y) = if a.y < b.y { (a.y, b.y) } else { (b.y, a.y) };

    // interior (exclusive)
    for y in min_y + 1..max_y {
        for x in min_x + 1..max_x {
            if matrix[y][x] != 'I' {
                return false;
            }
        }
    }

    true
}

// this was my solution but it's too slow.
fn _phase2_discarded(content: String) {
    let original_tiles = content
        .lines()
        .map(|f| Coord::from_str(f).expect("invalid coord"))
        .collect::<Vec<_>>();

    let mut tiles = original_tiles.clone();

    let max_x = tiles.iter().map(|c| c.x).max().unwrap();
    let max_y = tiles.iter().map(|c| c.y).max().unwrap();

    let mut matrix = create_matrix(max_y + 2, max_x + 2);

    for tile in &tiles {
        matrix[tile.y][tile.x] = '#';
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    let all_tiles = tiles.iter().copied().collect::<HashSet<Coord>>();

    while let Some(next) = tiles.pop() {
        matrix[next.y][next.x] = '#';
        for (dx, dy) in [(1isize, 0), (-1, 0), (0, 1), (0, -1)] {
            let mut acc = Vec::<Coord>::new();

            let mut nx = next.x as isize + dx;
            let mut ny = next.y as isize + dy;

            while nx >= 0 && nx < cols as isize && ny >= 0 && ny < rows as isize {
                if all_tiles.contains(&Coord::from((nx as usize, ny as usize))) {
                    for coord in acc {
                        matrix[coord.y][coord.x] = '#';
                    }
                    break;
                }

                acc.push(Coord::from((nx as usize, ny as usize)));

                nx += dx;
                ny += dy;
            }
        }
    }

    flood_fill(&mut matrix, 0, 0, &['.'], 'O');

    let first = all_tiles.iter().next().copied().unwrap();
    flood_fill(&mut matrix, first.x, first.y, &['.', '#'], 'I');

    let all = original_tiles
        .iter()
        .flat_map(|coord| {
            original_tiles
                .iter()
                .filter(|other| other != &coord && other < &coord)
                .map(|other| (*coord, *other, coord.area(other)))
                .collect::<Vec<_>>()
        })
        .sorted_by_key(|a| a.2)
        .rev()
        .find(|(a, b, _)| rect_is_all_I(*a, *b, &matrix))
        .map(|(_, _, area)| area);

    println!("ANS: {:?}", all);
}
