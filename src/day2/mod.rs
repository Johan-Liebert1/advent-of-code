/*
A = X = Rock -> Selected by your = 1 pts
B = Y = Paper -> Selected by your = 2 pts
C = Z = Scissors -> Selected by your = 3 pts

Loss = 0
Draw = 3
Win = 6
*/

const LOSE: i32 = 0;
const WIN: i32 = 6;
const DRAW: i32 = 3;

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

const OPP_ROCK: &str = "A";
const OPP_PAPER: &str = "B";
const OPP_SCISSORS: &str = "C";

const MY_ROCK: &str = "X";
const MY_PAPER: &str = "Y";
const MY_SCISSORS: &str = "Z";

const NEED_LOSE: &str = "X";
const NEED_DRAW: &str = "Y";
const NEED_WIN: &str = "Z";

use crate::helper::read_input;

pub fn part1() {
    let input = read_input(2);

    let vec: i32 = input
        .split("\n")
        .map(|inp| inp.split(" ").map(|i| i).collect::<Vec<&str>>())
        .map(|round_result| {
            if round_result.len() < 2 {
                return 0;
            }

            let opponent = round_result[0];
            let me = round_result[1];

            match opponent {
                // rock
                OPP_ROCK => match me {
                    MY_ROCK => DRAW + ROCK,
                    MY_PAPER => WIN + PAPER,
                    MY_SCISSORS => LOSE + SCISSORS,
                    _ => 0,
                },

                OPP_PAPER => match me {
                    MY_ROCK => LOSE + ROCK,
                    MY_PAPER => DRAW + PAPER,
                    MY_SCISSORS => WIN + SCISSORS,
                    _ => 0,
                },

                OPP_SCISSORS => match me {
                    MY_ROCK => WIN + ROCK,
                    MY_PAPER => LOSE + PAPER,
                    MY_SCISSORS => DRAW + SCISSORS,
                    _ => 0,
                },

                _ => 0,
            }
        })
        .sum();

    println!("Total Score: {}", vec);
}

pub fn part2() {
    let input = read_input(2);

    let vec: i32 = input
        .split("\n")
        .map(|inp| inp.split(" ").map(|i| i).collect::<Vec<&str>>())
        .map(|round_result| {
            if round_result.len() < 2 {
                return 0;
            }

            let opponent = round_result[0];
            let need_win_lose_or_draw = round_result[1];

            match opponent {
                // rock
                OPP_ROCK => match need_win_lose_or_draw {
                    NEED_WIN => WIN + PAPER,
                    NEED_DRAW => DRAW + ROCK,
                    NEED_LOSE => LOSE + SCISSORS,
                    _ => 0,
                },

                OPP_PAPER => match need_win_lose_or_draw {
                    NEED_WIN => WIN + SCISSORS,
                    NEED_DRAW => DRAW + PAPER,
                    NEED_LOSE => LOSE + ROCK,
                    _ => 0,
                },

                OPP_SCISSORS => match need_win_lose_or_draw {
                    NEED_WIN => WIN + ROCK,
                    NEED_DRAW => DRAW + SCISSORS,
                    NEED_LOSE => LOSE + PAPER,
                    _ => 0,
                },

                _ => 0,
            }
        })
        .sum();

    println!("Total Score: {}", vec);
}
