#![allow(dead_code, unused_parens)]

use std::env;

mod helper;
mod answers;

fn main() {
    let args: Vec<String> = env::args().collect();

    answers::day14::part2(if args.len() > 1 && args[1] == "test" { true } else { false });
}
