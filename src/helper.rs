use std::fs;

pub fn read_input(day: usize) -> String {
    fs::read_to_string(format!("src/day{}/input.txt", day)).unwrap()
}

