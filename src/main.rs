#![allow(dead_code, unused_parens)]

extern crate regex;

use std::env;

mod helper;
mod answers;

fn main() {
    let args: Vec<String> = env::args().collect();

    answers::day15::part1(if args.len() > 1 && args[1] == "test" { true } else { false });
}
