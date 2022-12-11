struct Monkey {
    starting_items: Vec<u128>,

    // what operation does the monkey do to the item
    operation: Box<dyn Fn(u128) -> u128>,

    // tests and returns which monkey to send the item to
    test: Box<dyn Fn(u128) -> usize>,
}

fn get_input() -> ([Monkey; 8], u128) {
    (
        [
            (Monkey {
                starting_items: vec![66, 71, 94],
                operation: Box::new(|item: u128| {
                    return item * 5;
                }),
                test: Box::new(|item: u128| if item % 3 == 0 { 7 } else { 4 }),
            }),
            (Monkey {
                starting_items: vec![70],
                operation: Box::new(|item: u128| {
                    return item + 6;
                }),
                test: Box::new(
                    |item: u128| {
                        if item % 17 == 0 {
                            3
                        } else {
                            0
                        }
                    },
                ),
            }),
            (Monkey {
                starting_items: vec![62, 68, 56, 65, 94, 78],
                operation: Box::new(|item: u128| {
                    return item + 5;
                }),
                test: Box::new(|item: u128| if item % 2 == 0 { 3 } else { 1 }),
            }),
            (Monkey {
                starting_items: vec![89, 94, 94, 67],
                operation: Box::new(|item: u128| {
                    return item + 2;
                }),
                test: Box::new(
                    |item: u128| {
                        if item % 19 == 0 {
                            7
                        } else {
                            0
                        }
                    },
                ),
            }),
            (Monkey {
                starting_items: vec![71, 61, 73, 65, 98, 98, 63],
                operation: Box::new(|item: u128| {
                    return item * 7;
                }),
                test: Box::new(|item: u128| if item % 11 == 0 { 5 } else { 6 }),
            }),
            (Monkey {
                starting_items: vec![55, 62, 68, 61, 60],
                operation: Box::new(|item: u128| {
                    return item + 7;
                }),
                test: Box::new(|item: u128| if item % 5 == 0 { 2 } else { 1 }),
            }),
            (Monkey {
                starting_items: vec![93, 91, 69, 64, 72, 89, 50, 71],
                operation: Box::new(|item: u128| {
                    return item + 1;
                }),
                test: Box::new(
                    |item: u128| {
                        if item % 13 == 0 {
                            5
                        } else {
                            2
                        }
                    },
                ),
            }),
            (Monkey {
                starting_items: vec![76, 50],
                operation: Box::new(|item: u128| {
                    return item * item;
                }),
                test: Box::new(
                    |item: u128| {
                        if item % 7 == 0 {
                            4
                        } else {
                            6
                        }
                    },
                ),
            }),
        ],
        (3 * 17 * 2 * 19 * 11 * 5 * 13 * 7) as u128,
    )
}

pub fn part1() {
    let part = 2;

    // worry level is divided by 3 and rounded down after each operation
    let (mut monkeys, grand_father_modulo) = get_input();

    let mut times_monkey_inspected = [0 as u64; 8];

    let range = if part == 1 { 20 } else { 10_000 };

    for _ in 0..range {
        let mut new_monkey_items: Vec<Vec<u128>> = vec![];

        for _ in 0..monkeys.len() {
            new_monkey_items.push(vec![]);
        }

        for i in 0..monkeys.len() {
            let (monkeys_left, monkeys_mid) = monkeys.split_at_mut(i);
            let (monkey, monkeys_right) = monkeys_mid.split_at_mut(1);

            let monkey = &mut monkey[0];

            while monkey.starting_items.len() > 0 {
                let item = &monkey.starting_items[0];

                let mut worry_level = (monkey.operation)(*item);
                let throw_to_monkey = (monkey.test)(worry_level);

                if true {
                    worry_level %= grand_father_modulo;
                }

                if throw_to_monkey > i {
                    let other_monkey = &mut monkeys_right[throw_to_monkey - i - 1];
                    other_monkey.starting_items.push(worry_level);
                } else {
                    let other_monkey = &mut monkeys_left[throw_to_monkey];
                    other_monkey.starting_items.push(worry_level);
                }

                times_monkey_inspected[i] += 1;

                monkey.starting_items.remove(0);
            }
        }
    }

    times_monkey_inspected.sort();

    println!(
        "{:?}",
        times_monkey_inspected[monkeys.len() - 2] * times_monkey_inspected[monkeys.len() - 1]
    );
}
