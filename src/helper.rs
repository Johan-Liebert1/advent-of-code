use std::fs;

pub fn read_input(day: usize) -> String {
    fs::read_to_string(format!("src/inputs/day{}.txt", day)).unwrap()
}

