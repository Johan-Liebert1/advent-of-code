use std::fs;

pub fn read_input(day: usize, test: bool) -> String {
    match test {
        false => fs::read_to_string(format!("src/inputs/day{}.txt", day)).unwrap(),
        true => fs::read_to_string(format!(
            "src/inputs{}/day{}.txt",
            "/test",
            day
        ))
        .unwrap(),
    }
}
