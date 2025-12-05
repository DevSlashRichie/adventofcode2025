use crate::utils::read_file_contents;

fn is_invalid_number(n: &str) -> bool {
    let mut current = String::from(&n[0..1]);
    let mut same_seq = false;

    let mut index = 1;
    while index < n.len() {
        if index == 0 {
            continue;
        }

        let chars_left = n.len() - index;

        if chars_left < current.len() {
            return false;
        }

        let end_inner = index + current.len();

        if end_inner > n.len() {
            break;
        }

        let sub = &n[index..end_inner];

        if sub != current {
            current = n[0..=index].to_owned();
            same_seq = false;
        } else {
            same_seq = true;
        }

        if end_inner == n.len() {
            break;
        }

        if same_seq {
            index += current.len();
            continue;
        }

        index += 1;
    }

    same_seq
}

fn is_invalid_number_1(n: &str) -> bool {
    if n.len() % 2 != 0 {
        return false;
    }

    let half = n.len() / 2;
    &n[0..half] == &n[half..n.len()]
}

pub fn _run() {
    let r = is_invalid_number("419441949");
    println!("PEPEPE: {r}");
}

pub fn run(second: bool) {
    // parse ranges
    let content = read_file_contents("day2.txt");
    let ranges = content
        .split(',')
        .map(|line| line.trim())
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            let (start, end): (u64, u64) = (start.parse().unwrap(), end.parse().unwrap());

            start..=end
        })
        .collect::<Vec<_>>();

    let total: u64 = ranges
        .into_iter()
        .flat_map(|range| range)
        .filter(|number| {
            if second {
                is_invalid_number(&number.to_string())
            } else {
                is_invalid_number_1(&number.to_string())
            }
        })
        .sum();

    println!("TOTAL: {total}");
}
