use crate::helper::read_input;

enum LineType<'a> {
    ChangeDir(&'a str),
    Dir(&'a str),
    Ls,
    FileSize(usize),
}

fn decode_line<'a>(line: &'a str) -> LineType<'a> {
    let mut char_iter = line.chars();

    match char_iter.next() {
        Some(char) => {
            if char == '$' {
                match char_iter.skip(1).next() {
                    Some(next_char) => {
                        if next_char == 'c' {
                            LineType::ChangeDir(&line[5..])
                        } else {
                            LineType::Ls
                        }
                    }
                    None => LineType::Ls,
                }
            } else if char.is_numeric() {
                LineType::FileSize(line.split(" ").next().unwrap().parse::<usize>().unwrap())
            } else {
                LineType::Dir(&line[1..])
            }
        }

        None => LineType::Ls,
    }
}

// keep calculating the current directory size until we get a cd commad
fn calculate_dir_size(
    split: &Vec<&str>,
    index: &mut usize,
    total_sum: &mut usize,
    current_dir: String
) -> usize {
    let mut current_dir_size = 0;

    while *index < split.len() {
        let line = split[*index];

        println!("{}", line);

        if line.len() == 0 {
            return current_dir_size;
        }

        match decode_line(&line) {
            LineType::ChangeDir(changed_to_dir) => {
                if changed_to_dir == ".." {
                    // println!("cd ..");
                    // get rid of the last dir as we have changed into its parent dir
                    // *index += 1;
                    break;
                } else {
                    // println!("cd {}", changed_to_dir);
            
                    *index += 1;
                    current_dir_size += calculate_dir_size(
                        split,
                        index,
                        total_sum,
                        (current_dir.to_owned() + &"/".to_string() + &changed_to_dir.to_string())
                    );
                }
            }

            LineType::Dir(_) => {}

            LineType::Ls => {}

            LineType::FileSize(size) => {
                current_dir_size += size;
            }
        }

        *index += 1;
    }

    // println!("{} size {}", current_dir, current_dir_size);

    // get rid of the last dir as we have changed into its parent dir
    if current_dir_size <= 100_000 {
        *total_sum += current_dir_size;
    }

    current_dir_size
}

pub fn part1() {
    let file = read_input(7, false);
    let split = file.split("\n").collect::<Vec<&str>>();

    let mut total_sum = 0;

    calculate_dir_size(&split, &mut 0, &mut total_sum, "".to_string());

    println!("{}", total_sum);
}
