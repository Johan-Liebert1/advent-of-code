use crate::helper;

use self::Op::{Add, NoOp};

enum Op {
    NoOp,
    Add(i32),
}

fn parse_line(line: &str) -> Op {
    let mut split = line.split(" ");

    match split.next() {
        None => NoOp,
        Some(thing) => {
            if thing == "noop" {
                return NoOp;
            }

            return Add(split.next().unwrap().parse().unwrap());
        }
    }
}

fn update_total(cycle: &i32, reg: &i32, total: &mut i32) {
    if *cycle == 20 {
        *total += reg * cycle;
    } else if (*cycle - 20) % 40 == 0 {
        *total += reg * cycle;
    }

    // println!("cycle: {}, reg: {}, total: {}", cycle, reg, total);
}

pub fn part1() {
    let file = helper::read_input(10, false);

    let mut cycle = 0;
    let mut total = 0;
    let mut reg = 1;

    for line in file.split("\n") {
        if line.len() == 0 {
            continue;
        }

        match parse_line(line) {
            NoOp => { 
                cycle += 1;
                update_total(&cycle, &reg, &mut total);
                continue;
            },
            Add(number) => {
                cycle += 1;
                update_total(&cycle, &reg, &mut total);

                cycle += 1;
                update_total(&cycle, &reg, &mut total);

                reg += number;
            }
        }
    }

    println!("{}", total);
}


pub fn part2() {
    let file = helper::read_input(10, false);

    let mut crt = [['.'; 40]; 6];

    let mut cycle = 0;
    let mut total = 0;
    let mut reg = 1;

    for line in file.split("\n") {
        if line.len() == 0 {
            continue;
        }

        match parse_line(line) {
            NoOp => { 
                cycle += 1;
                update_total(&cycle, &reg, &mut total);
                continue;
            },
            Add(number) => {
                cycle += 1;
                update_total(&cycle, &reg, &mut total);

                cycle += 1;
                update_total(&cycle, &reg, &mut total);

                reg += number;
            }
        }
    }

    println!("{}", total);
}
