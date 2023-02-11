use std::collections::{HashMap, HashSet};

use crate::helper::read_input;
use regex::Regex;

fn parse_input(test: bool) -> (HashMap<(i64, i64), (i64, i64)>, HashSet<(i64, i64)>, i64, i64) {
    let mut map = HashMap::new();

    let file = read_input(15, test);
    let (mut min_left, mut max_right) = (i64::MAX, 0);

    let mut unique_beacons = HashSet::new();

    let pattern =
        Regex::new(r#".*x=([-]*[\d]+).*y=([-]*[\d]+).*x=([-]*[\d]+).*y=([-]*[\d]+)"#).unwrap();

    for line in file.split("\n") {
        for capture in pattern.captures_iter(line) {
            let sensor_x = capture.get(1).unwrap();
            let sensor_y = capture.get(2).unwrap();
            let beacon_x = capture.get(3).unwrap();
            let beacon_y = capture.get(4).unwrap();

            let sensor_x = &line[sensor_x.start()..sensor_x.end()];
            let sensor_y = &line[sensor_y.start()..sensor_y.end()];
            let beacon_x = &line[beacon_x.start()..beacon_x.end()];
            let beacon_y = &line[beacon_y.start()..beacon_y.end()];

            let sensor_x = sensor_x.parse::<i64>().unwrap();
            let sensor_y = sensor_y.parse::<i64>().unwrap();
            let beacon_x = beacon_x.parse::<i64>().unwrap();
            let beacon_y = beacon_y.parse::<i64>().unwrap();

            min_left = if sensor_x.min(beacon_x) < min_left {
                sensor_x.min(beacon_x)
            } else {
                min_left
            };
            max_right = if sensor_x.max(beacon_x) > max_right {
                sensor_x.max(beacon_x)
            } else {
                max_right
            };

            unique_beacons.insert((beacon_x, beacon_y));
            map.insert((sensor_x, sensor_y), (beacon_x, beacon_y));
        }
    }

    (map, unique_beacons, min_left, max_right)
}

fn get_neighbors(x: i64, y: i64, row_to_check: i64) -> Vec<(i64, i64)> {
    let mut neighbors = vec![];

    let row_adders: [i32; 3] = [1, 0, -1];
    let col_adders: Vec<Vec<i32>> = vec![vec![-1, 0, 1], vec![-1, 1], vec![-1, 0, -1]];

    for i in 0..row_adders.len() {
        let ra = row_adders[i];

        for ca in &col_adders[i] {
            let neighbor_y = y + ra as i64;
            let neighbor_x = x + *ca as i64;

            if y < row_to_check {
                if neighbor_y >= y {
                    neighbors.push((neighbor_x, neighbor_y));
                }
            } else {
                if neighbor_y <= y {
                    neighbors.push((neighbor_x, neighbor_y));
                }
            }
        }
    }

    neighbors
}

fn dist(pt1: &(i64, i64), pt2: &(i64, i64)) -> f64 {
    let (sx, sy) = pt1;
    let (bx, by) = pt2;

    (((sx - bx).pow(2) + (sy - by).pow(2)) as f64).sqrt()
}

pub fn part1(test: bool) {
    let (sensor_beacon_map, unique_beacons, min_left, max_right) = parse_input(test);

    let row_to_check = if test { 10 } else { 2_000_000 };

    let mut visited = HashSet::new();
    let mut total = 0;

    for (sensor, beacon) in &sensor_beacon_map {
        let closest_distance = dist(sensor, beacon);

        let min_sensor_dist_to_row = dist(sensor, &(sensor.0, row_to_check));

        if min_sensor_dist_to_row > closest_distance {
            continue;
        }

        for x in min_left..max_right + 1 {
            let point = (x, row_to_check);

            // measure the distance of every beacon from every point on this line
            let point_dist_from_sensor = dist(sensor, &point);

            if point_dist_from_sensor <= closest_distance && visited.get(&point).is_none() {
                visited.insert(point);
                total += 1;
            }
        }
    }

    for (_, by) in unique_beacons {
        if by == row_to_check {
            total -= 1;
        }
    }

    println!("{}", total);
}
