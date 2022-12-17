use self::PacketContainer::{InnerPacketContainer, Int};
use crate::helper::read_input;

#[derive(Debug)]
enum PacketContainer {
    Int(u32),
    InnerPacketContainer(Packet),
}

type Packet = Vec<PacketContainer>;

const TEST: bool = false;

fn fill_packet(iterator: &mut impl Iterator<Item = char>, packet: &mut Packet) {
    match iterator.next() {
        None => return,

        Some(character) => {
            if character == ']' {
                return;
            }

            if character != ',' && character != '\n' {
                if character != '[' {
                    packet.push(Int(character.to_digit(10).unwrap()));
                } else {
                    let mut new_packet: Packet = vec![];
                    fill_packet(iterator, &mut new_packet);
                    packet.push(InnerPacketContainer(new_packet));
                }
            }

            fill_packet(iterator, packet);
        }
    }
}

fn parse_line(line: &str) -> Packet {
    let mut p: Packet = vec![];

    let mut i = line.chars().skip(1);

    fill_packet(&mut i, &mut p);

    p
}

fn parse_input() -> Vec<Vec<Packet>> {
    let file = read_input(13, TEST);

    let mut packets = vec![];

    for line in file.split("\n\n") {
        let mut pp: Vec<Packet> = vec![];

        for input in line.split("\n") {
            pp.push(parse_line(input));
        }

        packets.push(pp);
    }

    packets
}

fn compare_packets<'a>(
    left: &mut impl Iterator<Item = &'a PacketContainer>,
    right: &mut impl Iterator<Item = &'a PacketContainer>,
) -> (bool) {
    let are_matching: bool;

    if let Some(next_left) = left.next() {
        if let Some(next_right) = right.next() {
            match next_left {
                Int(left_int) => match next_right {
                    Int(right_int) => {
                        if left_int < right_int {
                            are_matching = true;
                        } else if left_int > right_int {
                            are_matching = false;
                        } else {
                            are_matching = compare_packets(left, right);
                        }
                    }

                    InnerPacketContainer(right_packet) => {
                        are_matching = compare_packets(
                            &mut vec![Int(*left_int)].iter(),
                            &mut right_packet.iter(),
                        );
                    }
                },

                InnerPacketContainer(left_packet_container) => match next_right {
                    Int(right_int) => {
                        are_matching = compare_packets(
                            &mut left_packet_container.iter(),
                            &mut vec![Int(*right_int)].iter(),
                        )
                    }

                    InnerPacketContainer(right_packet_container) => {
                        are_matching = compare_packets(
                            &mut left_packet_container.iter(),
                            &mut right_packet_container.iter(),
                        )
                    }
                },
            }
        } else {
            // left.next() exists, but right.next() does not
            are_matching = false;
        }
    } else {
        are_matching = right.peekable().next().is_some();
    }

    return are_matching;
}

pub fn part1() {
    let packets = parse_input();
    let mut packets_iter = packets.iter();

    let mut packet_index = 0;
    let mut total_total = 0;

    while let Some(packet_pair) = &packets_iter.next() {
        packet_index += 1;

        let mut packet_pair_iter = packet_pair.iter();

        // println!("packet_index {}", packet_index);
        let left_packet = packet_pair_iter.next().unwrap();
        let right_packet = packet_pair_iter.next().unwrap();

        let compared_packets = compare_packets(&mut left_packet.iter(), &mut right_packet.iter());

        if compared_packets {
            total_total += packet_index;
        }

        // println!(
        //     "Packet number {} are in order {}",
        //     packet_index, compared_packets
        // );
        // println!("------------------------------");
        // println!("");
    }

    println!("{}", total_total);
}
