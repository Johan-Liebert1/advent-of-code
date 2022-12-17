use std::{collections::HashSet, thread, time::Duration};

use crate::helper::read_input;

type SetType = HashSet<(i32, i32)>;

fn parse_input(test: bool) -> (SetType, i32, i32, i32) {
    let mut min_left = i32::MAX;
    let mut max_right = 0;
    let mut max_y = 0;
    let mut set = HashSet::new();

    let file = read_input(14, test);

    for line in file.split("\n") {
        if line.len() == 0 {
            continue;
        }

        let paths = line.split(" -> ").collect::<Vec<&str>>();

        for i in 0..paths.len() - 1 {
            let mut current_xy = paths[i].split(",");
            let mut next_xy = paths[i + 1].split(",");

            let cx = current_xy.next().unwrap().parse::<i32>().unwrap();
            let cy = current_xy.next().unwrap().parse::<i32>().unwrap();

            let nx = next_xy.next().unwrap().parse::<i32>().unwrap();
            let ny = next_xy.next().unwrap().parse::<i32>().unwrap();

            let (cx, nx) = if cx < nx { (cx, nx) } else { (nx, cx) };
            let (cy, ny) = if cy < ny { (cy, ny) } else { (ny, cy) };

            for x in cx..nx + 1 {
                min_left = if x < min_left { x } else { min_left };
                max_right = if x > max_right { x } else { max_right };

                set.insert((x, cy));
                // set.insert((x, ny));
            }

            for y in cy..ny + 1 {
                set.insert((cx, y));
                max_y = if y > max_y { y } else { max_y };
                // set.insert((cy, y));
            }
        }
    }

    (set, min_left, max_right, max_y)
}

pub fn part1(test: bool) {
    let (mut occupied_coordinates, min_left, max_right, max_y) = parse_input(test);

    println!(
        "occupied_coordinates {} min_left {} max_right {} max_y {}",
        occupied_coordinates.len(),
        min_left,
        max_right,
        max_y
    );

    let sand_start = (500, 0);

    let mut current_sand_pos = sand_start;

    let mut total_sand_particles = 0;

    loop {
        if current_sand_pos.1 > max_y {
            break;
        }

        let next_sand_pos = (current_sand_pos.0, current_sand_pos.1 + 1);

        let can_sand_fall = occupied_coordinates.get(&next_sand_pos).is_none();

        if can_sand_fall {
            current_sand_pos.1 += 1;
        } else {
            let left_blocked = occupied_coordinates
                .get(&(current_sand_pos.0 - 1, current_sand_pos.1 + 1))
                .is_some();

            let right_blocked = occupied_coordinates
                .get(&(current_sand_pos.0 + 1, current_sand_pos.1 + 1))
                .is_some();

            if left_blocked && right_blocked {
                occupied_coordinates.insert(current_sand_pos);
                current_sand_pos = sand_start;
                total_sand_particles += 1;
            } else if !left_blocked {
                if current_sand_pos.0 - 1 < min_left {
                    break;
                }

                current_sand_pos.0 -= 1;
                current_sand_pos.1 += 1;
            } else if !right_blocked {
                if current_sand_pos.0 + 1 > max_right {
                    break;
                }

                current_sand_pos.0 += 1;
                current_sand_pos.1 += 1;
            }
        }
    }

    println!("{}", total_sand_particles);
}


pub fn part2(test: bool) {
    let (mut occupied_coordinates, min_left, max_right, mut max_y) = parse_input(test);

    max_y += 2;

    println!(
        "occupied_coordinates {} min_left {} max_right {} max_y {}",
        occupied_coordinates.len(),
        min_left,
        max_right,
        max_y
    );


    let sand_start = (500, 0);

    let mut current_sand_pos = sand_start;

    let mut total_sand_particles = 0;

    loop {
        if occupied_coordinates.get(&sand_start).is_some() {
            break;
        }

        let next_sand_pos = (current_sand_pos.0, current_sand_pos.1 + 1);

        let can_sand_fall = occupied_coordinates.get(&next_sand_pos).is_none() && current_sand_pos.1 < max_y - 1;

        if can_sand_fall {
            current_sand_pos.1 += 1;
        } else {
            let left_blocked = occupied_coordinates
                .get(&(current_sand_pos.0 - 1, current_sand_pos.1 + 1))
                .is_some();

            let right_blocked = occupied_coordinates
                .get(&(current_sand_pos.0 + 1, current_sand_pos.1 + 1))
                .is_some();

            if left_blocked && right_blocked {
                occupied_coordinates.insert(current_sand_pos);
                current_sand_pos = sand_start;
                total_sand_particles += 1;
            } else if !left_blocked {
                if current_sand_pos.1 + 1 == max_y {
                    occupied_coordinates.insert(current_sand_pos);
                    current_sand_pos = sand_start;
                    total_sand_particles += 1;
                    continue;
                }

                current_sand_pos.0 -= 1;
                current_sand_pos.1 += 1;

            } else if !right_blocked {
                if current_sand_pos.1 + 1 == max_y {
                    occupied_coordinates.insert(current_sand_pos);
                    current_sand_pos = sand_start;
                    total_sand_particles += 1;
                    continue;
                }

                current_sand_pos.0 += 1;
                current_sand_pos.1 += 1;
            } else {
                println!("else");
            }
        }
    }

    println!("{}", total_sand_particles);
}
