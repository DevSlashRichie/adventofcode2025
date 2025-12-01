use std::{fmt::Display, str::FromStr};

use crate::utils::read_file_as_lines;

// the first day, I wanted to use all "tools" RUST gives me.
// Ik this is horrible to be read.

#[derive(Debug)]
enum Command {
    L,
    R,
}

impl ToString for Command {
    fn to_string(&self) -> String {
        match self {
            Command::L => "L",
            Command::R => "R",
        }
        .to_string()
    }
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

    let mut dial = 50i32;
    let mut counter = 0;
    let mut passes = 0;
    for rotation in all_rotations {
        let rotation_value = match rotation.command {
            Command::L => rotation.value * -1,
            Command::R => rotation.value,
        };

        // where to move dial.
        let delta = rotation_value % 100;

        // exceeded rotations.
        let exc = (rotation_value / 100).unsigned_abs();

        // new number
        let next = dial + delta;

        if next == 0 {
            passes += 1;
        } else if dial != 0 && next <= 0 || next > 99 {
            passes += 1;
        }

        //dial = ((next as u32) % 100u32) as i32;
        dial = next.rem_euclid(100);

        //dial = next;

        //if dial < 0 {
        //    dial += 100;
        //}

        //if dial > 99 {
        //    dial -= 100;
        //}

        passes += exc;

        if dial == 0 {
            counter += 1;
        }
    }

    println!("PHASE1: {counter} - PHASE2: {passes}");
}
