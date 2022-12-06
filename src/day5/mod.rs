use crate::helper::read_input;

pub fn part1() {
    let mut vector: Vec<Vec<char>> = vec![
        vec!['N', 'B', 'D', 'T', 'V', 'G', 'Z', 'J'],
        vec!['S', 'R', 'M', 'D', 'W', 'P', 'F'],
        vec!['V', 'C', 'R', 'S', 'Z'],
        vec!['R', 'T', 'J', 'Z', 'P', 'H', 'G'],
        vec!['T', 'C', 'J', 'N', 'D', 'Z', 'Q', 'F'],
        vec!['N', 'V', 'P', 'W', 'G', 'S', 'F', 'M'],
        vec!['G', 'C', 'V', 'B', 'P', 'Q'],
        vec!['Z', 'B', 'P', 'N'],
        vec!['W', 'P', 'J'],
    ];

    let file = read_input(5);

    for line in file.split("\n") {
        // move 2 from 4 to 6

        let mut move_from_to = vec![];

        for word in line.split(" ") {
            match word.parse::<usize>() {
                Err(_) => {}
                Ok(val) => move_from_to.push(val - 1),
            }
        }

        if move_from_to.len() == 0 {
            continue;
        }

        for _ in 0..(move_from_to[0] + 1) {
            match vector[move_from_to[1]].pop() {
                Some(char) => {
                    vector[move_from_to[2]].push(char);
                }

                None => {}
            }
        }
    }

    for vec in vector {
        print!("{}", vec.last().unwrap());
    }
}

pub fn part2() {
    let mut vector: Vec<Vec<char>> = vec![
        vec!['N', 'B', 'D', 'T', 'V', 'G', 'Z', 'J'],
        vec!['S', 'R', 'M', 'D', 'W', 'P', 'F'],
        vec!['V', 'C', 'R', 'S', 'Z'],
        vec!['R', 'T', 'J', 'Z', 'P', 'H', 'G'],
        vec!['T', 'C', 'J', 'N', 'D', 'Z', 'Q', 'F'],
        vec!['N', 'V', 'P', 'W', 'G', 'S', 'F', 'M'],
        vec!['G', 'C', 'V', 'B', 'P', 'Q'],
        vec!['Z', 'B', 'P', 'N'],
        vec!['W', 'P', 'J'],
    ];

    let file = read_input(5);

    for line in file.split("\n") {
        // move 2 from 4 to 6

        let mut move_from_to = vec![];

        for word in line.split(" ") {
            match word.parse::<usize>() {
                Err(_) => {}
                Ok(val) => move_from_to.push(val - 1),
            }
        }

        if move_from_to.len() == 0 {
            continue;
        }

        let mut temp_vec = vec![];

        for _ in 0..(move_from_to[0] + 1) {
            match vector[move_from_to[1]].pop() {
                Some(char) => {
                    temp_vec.push(char);
                }

                None => {}
            }
        }

        for char in temp_vec.iter().rev() {
            vector[move_from_to[2]].push(*char);
        }
    }

    for vec in vector {
        print!("{}", vec.last().unwrap());
    }
}
