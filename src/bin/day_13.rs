use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, PartialEq)]
enum PacketData {
    Integer(u32),
    NestedList(Vec<PacketData>),
}

fn parse_packet(input: &'static str) -> Vec<PacketData> {
    if input.len() <= 2 {
        Vec::new()
    } else {
        let elements_string = &input[1..input.len() - 1];
        let mut packet_elements = Vec::new();
        let mut current_element_start_index = 0;
        let mut nesting_depth = 0;
        for (i, c) in elements_string.chars().enumerate() {
            if c == ',' && nesting_depth == 0 {
                packet_elements.push(&elements_string[current_element_start_index..i]);
                current_element_start_index = i + 1
            } else if c == '[' {
                nesting_depth += 1;
            } else if c == ']' {
                nesting_depth -= 1;
            }
        }
        packet_elements.push(&elements_string[current_element_start_index..]);

        packet_elements
            .iter()
            .map(|d| {
                if d.starts_with('[') {
                    PacketData::NestedList(parse_packet(d))
                } else {
                    PacketData::Integer(d.parse::<u32>().unwrap())
                }
            })
            .collect()
    }
}

fn parse_input_as_pairs(input: &'static str) -> Vec<(Vec<PacketData>, Vec<PacketData>)> {
    input
        .split("\n\n")
        .map(|packet_pair| packet_pair.split_once('\n').unwrap())
        .map(|(p_1, p_2)| (parse_packet(p_1.trim()), parse_packet(p_2.trim())))
        .collect()
}

fn parse_inputs_individually(input: &'static str) -> Vec<Vec<PacketData>> {
    input
        .split_whitespace()
        .map(|line| parse_packet(line.trim()))
        .collect()
}

fn get_packet_ordering(left: &Vec<PacketData>, right: &Vec<PacketData>) -> Ordering {
    let mut left_iter = left.iter();
    let mut right_iter = right.iter();

    for _ in 0..left.len().max(right.len()) {
        match (left_iter.next(), right_iter.next()) {
            (Some(l), Some(r)) => {
                let comparison = match (l, r) {
                    (PacketData::Integer(integer_left), PacketData::Integer(integer_right)) => {
                        integer_left.cmp(integer_right)
                    }
                    (PacketData::Integer(_), PacketData::NestedList(right_list)) => {
                        get_packet_ordering(&vec![l.clone()], right_list)
                    }
                    (PacketData::NestedList(left_list), PacketData::Integer(_)) => {
                        get_packet_ordering(left_list, &vec![r.clone()])
                    }
                    (PacketData::NestedList(left_list), PacketData::NestedList(right_list)) => {
                        get_packet_ordering(left_list, right_list)
                    }
                };

                if comparison != Ordering::Equal {
                    return comparison;
                }
            }
            (Some(_), None) => {
                return Ordering::Greater;
            }
            (None, Some(_)) => return Ordering::Less,
            (None, None) => (),
        }
    }

    Ordering::Equal
}

fn main() {
    let input = include_str!("../inputs/data_day_13.txt");

    // Solution for puzzle 1
    let packet_pairs = parse_input_as_pairs(input);
    let sum_of_indices_in_correct_order: usize = packet_pairs
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| get_packet_ordering(left, right) == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum();
    println!(
        "The sum of packet pair indices, for packets that are in the right order, is {}",
        sum_of_indices_in_correct_order
    );

    // Solution for puzzle 2
    let mut packets = parse_inputs_individually(input);
    let separator_1 = parse_packet("[[2]]");
    packets.push(separator_1.clone());
    let separator_2 = parse_packet("[[6]]");
    packets.push(separator_2.clone());
    packets.sort_by(get_packet_ordering);
    let product_of_separator_indices: usize = packets
        .iter()
        .cloned()
        .enumerate()
        .filter(|(_, packet)| packet == &separator_1 || packet == &separator_2)
        .map(|(i, _)| i + 1)
        .product();
    println!(
        "The product of separator packet indices is {}",
        product_of_separator_indices
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec;

    #[test]
    fn test_packet_parsing() {
        assert_eq!(parse_packet("[]"), Vec::new());
        assert_eq!(parse_packet("[1]"), vec![PacketData::Integer(1)]);
        assert_eq!(
            parse_packet("[1,2,3]"),
            vec![
                PacketData::Integer(1),
                PacketData::Integer(2),
                PacketData::Integer(3),
            ]
        );
        assert_eq!(
            parse_packet("[1,[],3]"),
            vec![
                PacketData::Integer(1),
                PacketData::NestedList(Vec::new()),
                PacketData::Integer(3),
            ]
        );
        assert_eq!(
            parse_packet("[[1,[2,[]]],3]"),
            vec![
                PacketData::NestedList(vec![
                    PacketData::Integer(1),
                    PacketData::NestedList(vec![
                        PacketData::Integer(2),
                        PacketData::NestedList(Vec::new()),
                    ]),
                ]),
                PacketData::Integer(3),
            ]
        );
    }

    #[test]
    fn test_input_parsing_as_pairs() {
        let packet_pairs = parse_input_as_pairs(
            "[[4,4],4,4]
            [[4,4],4,4,4]

            [1,[2,[3,[4,[5,6,7]]]],8,9]
            [1,[2,[3,[4,[5,6,0]]]],8,9]",
        );
        assert_eq!(
            packet_pairs,
            vec![
                (
                    vec![
                        PacketData::NestedList(vec![
                            PacketData::Integer(4),
                            PacketData::Integer(4),
                        ]),
                        PacketData::Integer(4),
                        PacketData::Integer(4),
                    ],
                    vec![
                        PacketData::NestedList(vec![
                            PacketData::Integer(4),
                            PacketData::Integer(4),
                        ]),
                        PacketData::Integer(4),
                        PacketData::Integer(4),
                        PacketData::Integer(4),
                    ]
                ),
                (
                    vec![
                        PacketData::Integer(1),
                        PacketData::NestedList(vec![
                            PacketData::Integer(2),
                            PacketData::NestedList(vec![
                                PacketData::Integer(3),
                                PacketData::NestedList(vec![
                                    PacketData::Integer(4),
                                    PacketData::NestedList(vec![
                                        PacketData::Integer(5),
                                        PacketData::Integer(6),
                                        PacketData::Integer(7),
                                    ]),
                                ]),
                            ]),
                        ]),
                        PacketData::Integer(8),
                        PacketData::Integer(9),
                    ],
                    vec![
                        PacketData::Integer(1),
                        PacketData::NestedList(vec![
                            PacketData::Integer(2),
                            PacketData::NestedList(vec![
                                PacketData::Integer(3),
                                PacketData::NestedList(vec![
                                    PacketData::Integer(4),
                                    PacketData::NestedList(vec![
                                        PacketData::Integer(5),
                                        PacketData::Integer(6),
                                        PacketData::Integer(0),
                                    ]),
                                ]),
                            ]),
                        ]),
                        PacketData::Integer(8),
                        PacketData::Integer(9),
                    ]
                ),
            ]
        );
    }

    #[test]
    fn test_input_parsing_individually() {
        let packets = parse_inputs_individually(
            "[[4,4],4,4]
            [[4,4],4,4,4]

            [1,[2,[3,[4,[5,6,7]]]],8,9]
            [1,[2,[3,[4,[5,6,0]]]],8,9]",
        );
        assert_eq!(
            packets,
            vec![
                vec![
                    PacketData::NestedList(vec![PacketData::Integer(4), PacketData::Integer(4),]),
                    PacketData::Integer(4),
                    PacketData::Integer(4),
                ],
                vec![
                    PacketData::NestedList(vec![PacketData::Integer(4), PacketData::Integer(4),]),
                    PacketData::Integer(4),
                    PacketData::Integer(4),
                    PacketData::Integer(4),
                ],
                vec![
                    PacketData::Integer(1),
                    PacketData::NestedList(vec![
                        PacketData::Integer(2),
                        PacketData::NestedList(vec![
                            PacketData::Integer(3),
                            PacketData::NestedList(vec![
                                PacketData::Integer(4),
                                PacketData::NestedList(vec![
                                    PacketData::Integer(5),
                                    PacketData::Integer(6),
                                    PacketData::Integer(7),
                                ]),
                            ]),
                        ]),
                    ]),
                    PacketData::Integer(8),
                    PacketData::Integer(9),
                ],
                vec![
                    PacketData::Integer(1),
                    PacketData::NestedList(vec![
                        PacketData::Integer(2),
                        PacketData::NestedList(vec![
                            PacketData::Integer(3),
                            PacketData::NestedList(vec![
                                PacketData::Integer(4),
                                PacketData::NestedList(vec![
                                    PacketData::Integer(5),
                                    PacketData::Integer(6),
                                    PacketData::Integer(0),
                                ]),
                            ]),
                        ]),
                    ]),
                    PacketData::Integer(8),
                    PacketData::Integer(9),
                ]
            ]
        )
    }

    #[test]
    fn test_packet_ordering() {
        assert_eq!(
            get_packet_ordering(&parse_packet("[1,1,3,1,1]"), &parse_packet("[1,1,5,1,1]")),
            Ordering::Less
        );
        assert_eq!(
            get_packet_ordering(&parse_packet("[[1],[2,3,4]]"), &parse_packet("[[1],4]")),
            Ordering::Less
        );
        assert_eq!(
            get_packet_ordering(&parse_packet("[9]"), &parse_packet("[[8,7,6]]")),
            Ordering::Greater
        );
        assert_eq!(
            get_packet_ordering(&parse_packet("[[4,4],4,4]"), &parse_packet("[[4,4],4,4,4]")),
            Ordering::Less
        );
        assert_eq!(
            get_packet_ordering(&parse_packet("[7,7,7,7]"), &parse_packet("[7,7,7]")),
            Ordering::Greater
        );
        assert_eq!(
            get_packet_ordering(&parse_packet("[]"), &parse_packet("[3]")),
            Ordering::Less
        );
        assert_eq!(
            get_packet_ordering(&parse_packet("[[[]]]"), &parse_packet("[[]]")),
            Ordering::Greater
        );
        assert_eq!(
            get_packet_ordering(
                &parse_packet("[1,[2,[3,[4,[5,6,7]]]],8,9]"),
                &parse_packet("[1,[2,[3,[4,[5,6,0]]]],8,9]"),
            ),
            Ordering::Greater
        );
    }
}
