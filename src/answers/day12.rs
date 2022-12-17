use std::collections::HashSet;

use crate::helper::read_input;

type Input = (Vec<Vec<GridCell>>, (usize, usize), (usize, usize));
type InputRef<'a> = (
    &'a mut Vec<Vec<GridCell>>,
    (usize, usize),
    (usize, usize),
);

#[derive(Debug, Clone)]
struct GridCell {
    row: usize,
    col: usize,
    value: u8,
    prev_node: Option<(usize, usize)>,
    is_visited: bool,
}

fn parse_input(is_test: bool) -> Input {
    let input = read_input(12, is_test);

    let (mut start, mut end) = ((0, 0), (0, 0));

    let vector: Vec<Vec<GridCell>> = input
        .split("\n")
        .enumerate()
        .map(|(line_index, line)| {
            line.chars()
                .enumerate()
                .map(|(char_index, char)| {
                    if char == 'S' {
                        start.0 = line_index;
                        start.1 = char_index;

                        return GridCell {
                            row: line_index,
                            col: char_index,
                            value: 244,
                            prev_node: None,
                            is_visited: false,
                        };
                    } else if char == 'E' {
                        end.0 = line_index;
                        end.1 = char_index;

                        return GridCell {
                            row: line_index,
                            col: char_index,
                            value: 'z' as u8,
                            prev_node: None,
                            is_visited: false,
                        };
                    }

                    return GridCell {
                        row: line_index,
                        col: char_index,
                        value: char as u8,
                        prev_node: None,
                        is_visited: false,
                    };
                })
                .collect()
        })
        .collect();

    // for v in &vector {
    //     for c in v {
    //         print!("{c:>4}", c=c);
    //     }

    //     println!();
    // }

    return (vector[..vector.len() - 1].to_vec(), start, end);
}

fn get_neighbors(vector: &Vec<Vec<GridCell>>, row: i32, col: i32) -> Vec<(usize, usize)> {
    let mut neighbors = vec![];

    let adders = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    let current_node = &vector[row as usize][col as usize];

    for adder in adders {
        let (ra, ca) = adder;

        let new_row = row + ra;
        let new_col = col + ca;

        if new_row >= 0 && (new_row as usize) < vector.len() {
            // within bounds
            if new_col >= 0 && (new_col as usize) < vector[0].len() {
                let new_node = &vector[new_row as usize][new_col as usize];

                if new_node.value <= current_node.value + 1 {
                    // println!("Adding new_node {}, current_node {}", new_node, current_node);
                    neighbors.push((new_row as usize, new_col as usize));
                }
            }
        }
    }

    neighbors
}

fn part1_len(vector: &Vec<Vec<GridCell>>, er: usize, ec: usize) -> i32 {
    let mut path_len = 0;
    let mut current_node = &vector[er][ec];

    while let Some(prev_node_coordinates) =  current_node.prev_node {
        path_len += 1;

        let (prev_node_row, prev_node_col) = prev_node_coordinates;
        current_node = &vector[prev_node_row][prev_node_col];
    }

    path_len
}


fn part2_len(vector: &Vec<Vec<GridCell>>, er: usize, ec: usize, all_starting_points: Vec<(usize, usize)>) -> i32 {
    let mut path_len = 0;
    let mut all_path_lens = vec![];
    let mut current_node = &vector[er][ec];

    while let Some(prev_node_coordinates) =  current_node.prev_node {
        path_len += 1;

        println!("{:?}", current_node);
        if all_starting_points.contains(&(current_node.row, current_node.col)) {
            all_path_lens.push(path_len);
        }

        let (prev_node_row, prev_node_col) = prev_node_coordinates;
        current_node = &vector[prev_node_row][prev_node_col];
    }

    println!("{:?}", all_path_lens);

    *all_path_lens.iter().min().unwrap()
}

fn bfs_shortest_path(input: InputRef, all_starting_points: Option<Vec<(usize, usize)>>) -> i32 {
    let (vector, (sr, sc), (er, ec)) = input;

    let mut visited = HashSet::new();
    let mut queue = vec![(sr, sc)];

    while queue.len() > 0 {
        let current_node = queue[0];

        visited.insert(current_node);

        let (row, col) = current_node;

        if row == er && col == ec {
            break;
        }

        for neigbor in get_neighbors(vector, row as i32, col as i32) {
            if visited.get(&neigbor).is_none() {
                // only if neighbor has not been visited
                queue.push(neigbor);
                visited.insert(neigbor);

                vector[neigbor.0][neigbor.1].prev_node = Some((row, col));
            }
        }

        queue = queue[1..].to_vec();
    }

    if let Some(all_starting_points) = all_starting_points {
        part2_len(&vector, er, ec, all_starting_points)
    } else {
        part1_len(&vector, er, ec)
    }
}

fn clean_vector(vector: &mut Vec<Vec<GridCell>>) {
    for vv in vector {
        for v in vv {
            v.prev_node = None;
        }
    }
}

pub fn part1(is_test: bool) {
    let (mut v, s, e) = parse_input(is_test);
    bfs_shortest_path((&mut v, s, e), None);
}


pub fn part2(is_test: bool) {
    let (mut vector, s, e) = parse_input(is_test);

    let mut all_starting_points = vec![];
    let mut queue = vec![s];
    let mut visited = HashSet::new();

    while queue.len() > 0 {
        let (row, col) = queue[0];

        for neigbor in get_neighbors(&vector, row as i32, col as i32) {
            if visited.get(&neigbor).is_none() && vector[neigbor.0][neigbor.1].value == 'a' as u8 {
                all_starting_points.push(neigbor);
                queue.push(neigbor);
                visited.insert(neigbor);
            }
        }

        queue = queue[1..].to_vec();
    }

    let mut min = i32::MAX;

    for start in all_starting_points {
        let thing = bfs_shortest_path((&mut vector, start, e), None);
        clean_vector(&mut vector);
        min = if thing < min { thing } else { min };
    }

    println!("{}", min);

}
