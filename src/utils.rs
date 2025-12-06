use std::{fs, path::PathBuf, str::FromStr};

pub fn read_file_as_lines(file_name: &str) -> Vec<String> {
    let path = PathBuf::from_str("inputs").unwrap().join(file_name);

    let contents = fs::read_to_string(path).expect(&format!("could not read file: {file_name}"));

    contents.split("\n").map(str::to_owned).collect()
}

pub fn read_file_contents(file_name: &str) -> String {
    let path = PathBuf::from_str("inputs").unwrap().join(file_name);

    let contents = fs::read_to_string(path).expect(&format!("could not read file: {file_name}"));
    contents
}

use std::fmt::Display;

pub fn print_matrix<T: Display>(matrix: &Vec<Vec<T>>) {
    for row in matrix {
        for val in row {
            print!("{val} ");
        }
        println!();
    }
}
