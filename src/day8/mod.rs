use crate::helper::read_input;

fn check_if_visible(vector: &Vec<Vec<u32>>, i: usize, j: usize) -> bool {
    let (mut top_visible, mut bottom_visible, mut left_visible, mut right_visible) = (true, true, true, true);

    // check top
    for row in 0..i {
        if vector[row][j] >= vector[i][j] {
            top_visible = false;
            break;
        }
    }

    // check bottom
    for row in i+1..vector.len() {
        if vector[row][j] >= vector[i][j] {
            bottom_visible = false;
            break;
        }
    }

    // check left
    for col in 0..j {
        if vector[i][col] >= vector[i][j] {
            left_visible = false;
            break;
        }
    }

    // check right
    for col in j+1..vector[0].len() {
        if vector[i][col] >= vector[i][j] {
            right_visible = false;
            break;
        }
    }

    return top_visible || bottom_visible || right_visible || left_visible;
}


fn scenic_score(vector: &Vec<Vec<u32>>, i: usize, j: usize) -> usize {
    if i == 0 || j == 0 || i == vector.len() - 1 || j == vector[0].len() {
        return 0;
    }


    let (mut top, mut bottom, mut left, mut right) = (0, 0, 0, 0);

    // check top
    for row in (0..i).rev() {
        top += 1;
        if vector[row][j] >= vector[i][j] {
            break;
        }
    }

    // check bottom
    for row in i+1..vector.len() {
        bottom += 1;
        if vector[row][j] >= vector[i][j] {
            break;
        }
    }

    // check left
    for col in (0..j).rev() {
        left += 1;
        if vector[i][col] >= vector[i][j] {
            break;
        }
    }

    // check right
    for col in j+1..vector[0].len() {
        right += 1;
        if vector[i][col] >= vector[i][j] {
            break;
        }
    }

    return top * bottom * left * right;
}

fn get_vector() -> Vec<Vec<u32>> {
    let file = read_input(8);

    let mut vector = file
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|character| character.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    vector.pop();

    return vector;
}

pub fn part1() {
    let vector = get_vector();

    let mut num_trees_visible = 0;

    // brute force
    for i in 0..vector.len() {
        for j in 0..vector[0].len() {
            if check_if_visible(&vector, i, j) {
                num_trees_visible += 1;
            }
        }
    }

    println!("{}", num_trees_visible);
}


pub fn part2() {
    let vector = get_vector();

    let mut max_scenic_score = 0;

    // brute force
    for i in 0..vector.len() {
        for j in 0..vector[0].len() {
            let scenic_score_ = scenic_score(&vector, i, j);

            max_scenic_score = if scenic_score_ > max_scenic_score { scenic_score_ } else { max_scenic_score };
        }
    }

    println!("{}", max_scenic_score);
}
