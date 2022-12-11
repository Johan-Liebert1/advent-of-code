// assuming both head and tail start at (0, 0)

use std::collections::HashSet;

use crate::helper::read_input;

enum Movement {
    Left(usize),
    Right(usize),
    Up(usize),
    Down(usize),
    None,
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

fn parse_line(line: &str) -> Movement {
    let mut split = line.split(" ");

    match split.next() {
        None => Movement::None,

        Some(dir) => {
            let steps: usize;

            match split.next() {
                None => return Movement::None,
                Some(number) => {
                    steps = number.parse::<usize>().unwrap();
                }
            }

            match dir {
                "R" => Movement::Right(steps),
                "L" => Movement::Left(steps),
                "U" => Movement::Up(steps),
                "D" => Movement::Down(steps),
                _ => Movement::None,
            }
        }
    }
}

fn move_tail(head: &Pos, tail: &mut Pos) {
    // same row
    if head.x == tail.x {
        if (head.y).abs_diff(tail.y) <= 1 {
            return;
        }

        // head up of tail
        if head.y > tail.y {
            tail.y += 1;
        } else {
            tail.y -= 1;
        }
    }

    // same col
    if head.y == tail.y {
        if (head.x).abs_diff(tail.x) <= 1 {
            return;
        }

        // head up of tail
        if head.x > tail.x {
            tail.x += 1;
        } else {
            tail.x -= 1;
        }
    }

    // diagonal somewhere
    if (head.y).abs_diff(tail.y) <= 1 && (head.x).abs_diff(tail.x) <= 1 {
        return;
    }

    if head.x > tail.x {
        if head.y > tail.y {
            // top right
            tail.x += 1;
            tail.y += 1;
        } else {
            // bottom right
            tail.x += 1;
            tail.y -= 1;
        }
    } else {
        if head.y > tail.y {
            // bottom left 
            tail.x -= 1;
            tail.y += 1;
        } else {
            // top left
            tail.x -= 1;
            tail.y -= 1;
        }
    }
}


fn move_tail_part2(vector: &mut Vec<Pos>, head_pos: usize, tail_pos: usize) {
    let head = vector[head_pos];
    let tail = &mut vector[tail_pos];

    // same row
    if head.x == tail.x {
        if (head.y).abs_diff(tail.y) <= 1 {
            return;
        }

        // head up of tail
        if head.y > tail.y {
            tail.y += 1;
        } else {
            tail.y -= 1;
        }
    }

    // same col
    if head.y == tail.y {
        if (head.x).abs_diff(tail.x) <= 1 {
            return;
        }

        // head up of tail
        if head.x > tail.x {
            tail.x += 1;
        } else {
            tail.x -= 1;
        }
    }

    // diagonal somewhere
    if (head.y).abs_diff(tail.y) <= 1 && (head.x).abs_diff(tail.x) <= 1 {
        return;
    }

    if head.x > tail.x {
        if head.y > tail.y {
            // top right
            tail.x += 1;
            tail.y += 1;
        } else {
            // bottom right
            tail.x += 1;
            tail.y -= 1;
        }
    } else {
        if head.y > tail.y {
            // bottom left 
            tail.x -= 1;
            tail.y += 1;
        } else {
            // top left
            tail.x -= 1;
            tail.y -= 1;
        }
    }
}

pub fn part1() {
    let file = read_input(9, false);

    let mut set = HashSet::new();
    let mut current_head = Pos { x: 0, y: 0 };
    let mut current_tail = Pos { x: 0, y: 0 };

    for line in file.split("\n") {
        if line.len() == 0 {
            continue;
        }

        match parse_line(line) {
            Movement::Left(steps) => {
                for _ in 0..steps {
                    current_head.x -= 1;
                    move_tail(&current_head, &mut current_tail);
                    set.insert((current_tail.x, current_tail.y));
                }
            }

            Movement::Right(steps) => {
                for _ in 0..steps {
                    current_head.x += 1;
                    move_tail(&current_head, &mut current_tail);
                    set.insert((current_tail.x, current_tail.y));
                }
            }

            Movement::Up(steps) => {
                for _ in 0..steps {
                    current_head.y += 1;
                    move_tail(&current_head, &mut current_tail);
                    set.insert((current_tail.x, current_tail.y));
                }
            }

            Movement::Down(steps) => {
                for _ in 0..steps {
                    current_head.y -= 1;
                    move_tail(&current_head, &mut current_tail);
                    set.insert((current_tail.x, current_tail.y));
                }
            }

            Movement::None => {}
        }
    }

    println!("{}", set.len());
}


pub fn part2() {
    let file = read_input(9, false);

    let mut set = HashSet::new();
    let mut current_head = Pos { x: 0, y: 0 };
    let mut tails = vec![Pos { x: 0, y: 0 }].repeat(9);

    for line in file.split("\n") {
        if line.len() == 0 {
            continue;
        }

        match parse_line(line) {
            Movement::Left(steps) => {
                for _ in 0..steps {
                    current_head.x -= 1;

                    move_tail(&current_head, &mut tails[0]);

                    for i in 1..tails.len() {
                        move_tail_part2(&mut tails, i - 1, i);
                    }

                    set.insert((tails[8].x, tails[8].y));
                }
            }

            Movement::Right(steps) => {
                for _ in 0..steps {
                    current_head.x += 1;

                    move_tail(&current_head, &mut tails[0]);

                    for i in 1..tails.len() {
                        move_tail_part2(&mut tails, i - 1, i);
                    }

                    set.insert((tails[8].x, tails[8].y));
                }
            }

            Movement::Up(steps) => {
                for _ in 0..steps {
                    current_head.y += 1;

                    move_tail(&current_head, &mut tails[0]);

                    for i in 1..tails.len() {
                        move_tail_part2(&mut tails, i - 1, i);
                    }

                    set.insert((tails[8].x, tails[8].y));
                }
            }

            Movement::Down(steps) => {
                for _ in 0..steps {
                    current_head.y -= 1;
                    
                    move_tail(&current_head, &mut tails[0]);

                    for i in 1..tails.len() {
                        move_tail_part2(&mut tails, i - 1, i);
                    }

                    set.insert((tails[8].x, tails[8].y));
                }
            }

            Movement::None => {}
        }
    }

    println!("{}", set.len());
}
