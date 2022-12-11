use crate::helper::read_input;
use std::collections::HashMap;

const CHARS_FOR_PART_1: usize = 4;
const CHARS_FOR_PART_2: usize = 14;

pub fn main_calc(array: &mut [char]) {
    println!("array.len() {}", array.len());
    let file = read_input(6, false);

    let mut duplicate_chars: HashMap<char, i32> = HashMap::new();

    let mut index_into_array = 0;
    let mut chars_traversed = 0;

    'outer: for current_char in file.chars() {
        if let Some(times_appears) = duplicate_chars.get_mut(&current_char) {
            *times_appears += 1;
        } else {
            duplicate_chars.insert(current_char, 1);
        }

        if let Some(times_appears) = duplicate_chars.get_mut(&array[index_into_array]) {
            *times_appears -= 1;

            // remove the chars if it does not appear again
            if *times_appears == 0 {
                duplicate_chars.remove(&array[index_into_array]);
            }
        }

        array[index_into_array] = current_char;

        index_into_array = if index_into_array == array.len() - 1 {
            0
        } else {
            index_into_array + 1
        };

        chars_traversed += 1;

        if chars_traversed >= array.len() {
            let mut break_outer = true;
            // if length of hash map is 0, the we have our answer
            for (_, value) in &duplicate_chars {
                if *value != 1 {
                    break_outer = false;
                    break;
                }
            }

            if break_outer { break 'outer };
        }
    }

    println!("{}", chars_traversed);
}

pub fn part1() {
    let mut array = [' '; CHARS_FOR_PART_1];
    main_calc(&mut array);
}


pub fn part2() {
    let mut array = [' '; CHARS_FOR_PART_2];
    main_calc(&mut array);
}
