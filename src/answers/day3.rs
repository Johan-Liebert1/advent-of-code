/*
Each rucksack has two large compartments. All items of a given type are meant to go into exactly one of the two compartments. The Elf that did the packing failed to follow this rule for exactly one item type per rucksack.

The Elves have made a list of all of the items currently in each rucksack (your puzzle input), but they need your help finding the errors. Every item type is identified by a single lowercase or uppercase letter (that is, a and A refer to different types of items).

The list of items for each rucksack is given as characters all on a single line. A given rucksack always has the same number of items in each of its two compartments, so the first half of the characters represent items in the first compartment, while the second half of the characters represent items in the second compartment.

For example, suppose you have the following list of contents from six rucksacks:

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp, which means its first compartment contains the items vJrwpWtwJgWr,
while the second compartment contains the items hcsFMMfFFhFp. The only item type that appears in both compartments is lowercase p.

The second rucksack's compartments contain jqHRNqRjqzjGDLGL and rsFMfFZSrLrFZsSL.
The only item type that appears in both compartments is uppercase L.

The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg; the only common item type is uppercase P.

The fourth rucksack's compartments only share item type v.

The fifth rucksack's compartments only share item type t.

The sixth rucksack's compartments only share item type s.

To help prioritize item rearrangement, every item type can be converted to a priority:

Lowercase item types a through z have priorities 1 through 26.
Uppercase item types A through Z have priorities 27 through 52.
In the above example, the priority of the item type that appears in both compartments of each rucksack is
16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19 (s); the sum of these is 157.
*/

// hell

use crate::helper::read_input;

const SMALL_PRIORITY_START: usize = 96;
const CAPITAL_PRIORITY_START: usize = 38;

pub fn part1() {
    let file = read_input(3);

    let mut total_priority = 0;

    for line in file.split("\n") {
        let line_len = line.len();

        let first_half = &line[..(line_len / 2)];
        let second_half = &line[line_len / 2..];

        for char in first_half.chars() {
            if let Some(_) = second_half.find(char) {
                let char_ascii = char as usize;
                total_priority += (char_ascii)
                    - if char_ascii >= 97 {
                        SMALL_PRIORITY_START
                    } else {
                        CAPITAL_PRIORITY_START
                    };
                break;
            }
        }
    }

    println!("Total Priority: {}", total_priority);
}

pub fn part2() {
    let file = read_input(3);

    let mut lines_iter = file.split("\n");

    let mut total_priority = 0;

    loop {
        let test = lines_iter.next();

        match test {
            Some(first_line) => {
                if let Some(second_line) = lines_iter.next() {
                    if let Some(third_line) = lines_iter.next() {
                        for char in first_line.chars() {
                            if let Some(_) = second_line.find(char) {
                                if let Some(_) = third_line.find(char) {
                                    let char_ascii = char as usize;

                                    total_priority += char_ascii
                                        - if char_ascii >= 97 {
                                            SMALL_PRIORITY_START
                                        } else {
                                            CAPITAL_PRIORITY_START
                                        };

                                    break;
                                }
                            }
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            None => break,
        }
    }

    println!("Total Priority: {}", total_priority);
}
