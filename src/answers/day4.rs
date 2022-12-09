use std::str::Split;

use crate::helper;

pub fn part1() -> Option<()> {
    let file = helper::read_input(4);

    let mut contains = 0;

    for line in file.split("\n") {
        if line.len() == 0 {
            continue;
        }

        let mut thing = line.split(",");

        let left = thing.next()?;
        let right = thing.next()?;

        let mut left_split = left.split("-");
        let mut right_split = right.split("-");

        let left_left = left_split.next()?.parse::<i32>();
        let left_right = left_split.next()?.parse::<i32>();

        let right_left = right_split.next()?.parse::<i32>();
        let right_right = right_split.next()?.parse::<i32>();

        match left_left {
            Err(_) => {}

            Ok(ll) => match left_right {
                Ok(lr) => match right_left {
                    Ok(rl) => match right_right {
                        Ok(rr) => {
                            let max_right = if lr > rr { rr } else { lr };
                            let min_left = if ll < rl { ll } else { rl };

                            if lr >= rl && rr >= ll {
                                println!("Line: {}", &line);
                                contains += 1;
                            }
                        }

                        Err(_) => {}
                    },

                    Err(_) => {}
                },

                Err(_) => {}
            },
        }
    }

    println!("Contains: {}", contains);

    Some(())
}
