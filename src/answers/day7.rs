use std::collections::HashMap;

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

pub fn part1() {
    let file = read_input(7);

    let mut dir_sizes: HashMap<String, usize> = HashMap::new();
    let mut dir_stack: Vec<&str> = vec![];

    let mut total_sum = 0;

    let mut current_dir_size = vec![];

    for line in file.split("\n") {
        if line.len() == 0 {
            continue;
        }

        match decode_line(&line) {
            LineType::ChangeDir(dir) => {
                // println!("dir {}", dir);
                if dir == ".." {
                    let current_dir = dir_stack.join("/").to_owned();

                    match dir_stack.pop() {
                        None => {}

                        // popped the dir, i.e. we did "cd ..", which means we're getting out of
                        // the child dir. The child dir is already added to the hash map and
                        // now we will increment the size of the current dir, which is the last
                        // dir in the dir_stack by the size of the child directory
                        Some(_) => match current_dir_size.pop() {
                            None => {}
                            Some(current_dir_size_last) => {
                                if current_dir_size_last <= 100_000 {
                                    total_sum += current_dir_size_last;
                                }

                                let parent_size = dir_sizes.get_mut(&current_dir).unwrap();
                                *parent_size += current_dir_size_last;
                            }
                        },
                    }
                } else {
                    current_dir_size.push(0);

                    dir_stack.push(dir);

                    let dir_path = dir_stack.join("/");

                    dir_sizes.insert(dir_path.to_owned(), 0);
                }
            }
            LineType::Dir(_) => {}
            LineType::Ls => {}
            LineType::FileSize(size) => {
                *current_dir_size.last_mut().unwrap() += size;
            }
        }

        println!("HashMap: {:?}", dir_sizes);
        println!("dir_stack: {:?}", dir_stack);
        println!("current_dir_size: {:?}", current_dir_size);
        println!("");
    }


    println!("total_sum {}", total_sum);
}
