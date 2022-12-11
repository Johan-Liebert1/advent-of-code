use crate::helper::read_input;

enum LineType<'a> {
    ChangeDir(&'a str),
    Dir(&'a str),
    Ls,
    FileSize(usize),
}

const TOTAL_DISK_SIZE: usize = 70000000;
const SPACE_REQ_FOR_UPDATE: usize = 30000000;

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
    total_sum: &mut usize
) -> usize {
    let mut current_dir_size = 0;

    while *index < split.len() {
        let line = split[*index];

        if line.len() == 0 {
            return current_dir_size;
        }

        match decode_line(&line) {
            LineType::ChangeDir(changed_to_dir) => {
                if changed_to_dir == ".." {
                    break;
                } else {
                    *index += 1;
                    current_dir_size += calculate_dir_size(
                        split,
                        index,
                        total_sum
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

    // get rid of the last dir as we have changed into its parent dir
    if current_dir_size <= 100_000 {
        *total_sum += current_dir_size;
    }

    current_dir_size
}



// keep calculating the current directory size until we get a cd commad
fn get_smallest_dir_to_delete(
    split: &Vec<&str>,
    index: &mut usize,
    space_to_clear: &usize,
    smallest_size: &mut usize,
) -> usize {
    let mut current_dir_size = 0;

    while *index < split.len() {
        let line = split[*index];

        if line.len() == 0 {
            return current_dir_size;
        }

        match decode_line(&line) {
            LineType::ChangeDir(changed_to_dir) => {
                if changed_to_dir == ".." {
                    break;
                } else {
                    *index += 1;
                    current_dir_size += get_smallest_dir_to_delete(
                        split,
                        index,
                        space_to_clear,
                        smallest_size
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

    // get rid of the last dir as we have changed into its parent dir
    if current_dir_size >= *space_to_clear && current_dir_size < *smallest_size {
        *smallest_size = current_dir_size;
    }

    println!("current_dir_size = {}", current_dir_size);

    current_dir_size
}


pub fn part1() {
    let file = read_input(7, true);
    let split = file.split("\n").collect::<Vec<&str>>();

    let mut total_sum = 0;

    calculate_dir_size(&split, &mut 0, &mut total_sum,);

    println!("{}", total_sum);
}

pub fn part2() {
    let file = read_input(7, false);
    let split = file.split("\n").collect::<Vec<&str>>();

    let mut smallest_size = usize::MAX;
    let mut total_sum = 0;

    let root_size = calculate_dir_size(&split, &mut 0, &mut total_sum);

    let space_unused = TOTAL_DISK_SIZE - root_size;
    let space_required = SPACE_REQ_FOR_UPDATE - space_unused;

    println!("space_unused {}, space_required {}", space_unused, space_required);

    get_smallest_dir_to_delete(&split, &mut 0, &space_required, &mut smallest_size);

    println!("{}", smallest_size);
}
