use std::str::FromStr;

use crate::utils::read_file_as_lines;

// the first day, I wanted to use all "tools" RUST gives me.
// Ik this is horrible to be read.

#[derive(Debug)]
enum Command {
    L,
    R,
}

impl From<char> for Command {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::L,
            'R' => Self::R,
            _ => panic!("invalid command"),
        }
    }
}

#[derive(Debug)]
struct Rotation {
    command: Command,
    value: i32,
}

impl FromStr for Rotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.chars();

        let command: Command = s.next().unwrap().into();
        let number = s.collect::<String>();

        let value = i32::from_str(&number).map_err(|err| err.to_string())?;

        Ok(Self { command, value })
    }
}

pub fn run(second: bool) {
    let lines = read_file_as_lines("day1.txt");

    let all_rotations = lines
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|line| Rotation::from_str(&line))
        .collect::<Result<Vec<_>, _>>()
        .expect("invalid line command");

    let mut dial = 50u32;
    let mut counter = 0;
    let mut passes = 0;
    for rotation in all_rotations {
        //println!("NEXT COMMAND: {:?}", &rotation);

        let rotation_value = match rotation.command {
            Command::L => rotation.value * -1,
            Command::R => rotation.value,
        };

        let mut new_dial = dial as i32 + rotation_value;
        println!("{dial} {new_dial} {rotation_value}");

        if new_dial <= 0 && dial != 0 {
            passes += 1;
        }

        if new_dial < 0 {
            new_dial = (100 + new_dial) % 100;
        } else if new_dial > 99 {
            new_dial = new_dial % 100;

            if dial != 0 {
                passes += 1;
            }
        }

        passes += (rotation_value / 100).unsigned_abs();

        if new_dial == 0 {
            counter += 1;
        }

        dial = new_dial as u32;
        //println!("DIAL: {dial}\n");
    }

    println!("RESPONSE: {counter} - {passes}");
}
