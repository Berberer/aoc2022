use std::collections::HashSet;

fn parse_input(input: &'static str) -> Vec<(usize, char)> {
    input
        .trim()
        .chars()
        .enumerate()
        .map(|(n, c)| (n + 1, c))
        .collect()
}

fn find_start_marker(
    signal_datastream: &Vec<(usize, char)>,
    marker_size: usize,
) -> &[(usize, char)] {
    signal_datastream
        .windows(marker_size)
        .find(|candidate| {
            candidate
                .iter()
                .map(|(_, c)| *c)
                .collect::<HashSet<char>>()
                .len()
                == marker_size
        })
        .unwrap()
}

fn main() {
    let input = include_str!("../inputs/data_day_6.txt");
    let signal_datastream = parse_input(input);

    // Solution for puzzle 1
    let start_of_packet_marker = find_start_marker(&signal_datastream, 4);
    println!(
        "{} characters of the datastream need to be processed to find the packet start marker",
        start_of_packet_marker.last().unwrap().0
    );

    // Solution for puzzle 1
    let start_of_message_marker = find_start_marker(&signal_datastream, 14);
    println!(
        "{} characters of the datastream need to be processed to find the message start marker",
        start_of_message_marker.last().unwrap().0
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_parsing() {
        assert_eq!(parse_input("aBc"), vec![(1, 'a'), (2, 'B'), (3, 'c')]);
    }

    #[test]
    fn test_find_start_of_packet_marker() {
        let test_subjects = vec![
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        for (data, expected_result) in test_subjects {
            let datastream = parse_input(data);
            let start_of_packet_marker = find_start_marker(&datastream, 4);
            assert_eq!(start_of_packet_marker.last().unwrap().0, expected_result);
        }
    }

    #[test]
    fn test_find_start_of_message_marker() {
        let test_subjects = vec![
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];

        for (data, expected_result) in test_subjects {
            let datastream = parse_input(data);
            let start_of_packet_marker = find_start_marker(&datastream, 14);
            assert_eq!(start_of_packet_marker.last().unwrap().0, expected_result);
        }
    }
}
