use std::collections::HashSet;

fn parse_input(input: &'static str) -> Vec<(HashSet<char>, HashSet<char>)> {
    input
        .lines()
        .map(|line| (line.trim(), line.trim().len() / 2))
        .map(|(line, center)| (&line[..center], &line[center..]))
        .map(|(compartment_1, compartment_2)| {
            (get_item_set(compartment_1), get_item_set(compartment_2))
        })
        .collect()
}

fn get_item_set(all_items: &str) -> HashSet<char> {
    all_items.chars().collect()
}

fn get_common_item(rucksack_compartments: &Vec<HashSet<char>>) -> char {
    let mut common_items = rucksack_compartments[0].clone();
    for compartment in rucksack_compartments[1..].iter() {
        common_items = common_items.intersection(compartment).cloned().collect();
    }
    *common_items.iter().next().unwrap()
}

fn get_item_priority(item: char) -> u32 {
    if item.is_lowercase() {
        u32::from(item) - 96 // a is 97
    } else {
        u32::from(item) - 38 // A is 65 (65-27=38)
    }
}

fn get_common_item_priorities_sum(
    rucksack_compartments: &Vec<(HashSet<char>, HashSet<char>)>,
) -> u32 {
    rucksack_compartments
        .iter()
        .cloned()
        .map(|(compartment_1, compartment_2)| get_common_item(&vec![compartment_1, compartment_2]))
        .map(get_item_priority)
        .sum()
}

fn group_elf_rucksacks(rucksacks: &Vec<(HashSet<char>, HashSet<char>)>) -> Vec<Vec<HashSet<char>>> {
    let mut grouped_rucksacks = Vec::new();
    for elves_group in rucksacks.chunks(3) {
        let elves_group_rucksacks = elves_group
            .iter()
            .map(|(compartment_1, compartment_2)| {
                compartment_1.union(compartment_2).cloned().collect()
            })
            .collect::<Vec<HashSet<char>>>();
        grouped_rucksacks.push(elves_group_rucksacks);
    }
    grouped_rucksacks
}

fn find_badge_of_elf_group(group_rucksacks: &Vec<HashSet<char>>) -> char {
    get_common_item(group_rucksacks)
}

fn main() {
    let input = include_str!("../inputs/data_day_3.txt");
    let rucksack_compartments = parse_input(input);

    // Solution for puzzle 1
    let priorities_sum = get_common_item_priorities_sum(&rucksack_compartments);
    println!(
        "Sum of priorities of common items of rucksack compartments is {}",
        priorities_sum
    );

    // Solution for puzzle 2
    let group_rucksacks = group_elf_rucksacks(&rucksack_compartments);
    let group_badges: Vec<char> = group_rucksacks
        .iter()
        .map(find_badge_of_elf_group)
        .collect();
    let badge_priorities: Vec<u32> = group_badges
        .iter()
        .map(|badge| get_item_priority(*badge))
        .collect();
    println!(
        "Sum of priorities of elf group badges is {}",
        badge_priorities.iter().sum::<u32>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_parsing() {
        let rucksack_compartments = parse_input(
            "abCD
            EFGhij",
        );
        assert_eq!(
            rucksack_compartments,
            vec![
                (HashSet::from(['a', 'b']), HashSet::from(['C', 'D'])),
                (
                    HashSet::from(['E', 'F', 'G']),
                    HashSet::from(['h', 'i', 'j'])
                ),
            ]
        );
    }

    #[test]
    fn test_common_compartment_item() {
        assert_eq!(
            get_common_item(&vec![HashSet::from(['a']), HashSet::from(['a'])]),
            'a'
        );
        assert_eq!(
            get_common_item(&vec![HashSet::from(['a', 'b']), HashSet::from(['B', 'a'])]),
            'a'
        );
    }

    #[test]
    fn test_item_priorities() {
        assert_eq!(get_item_priority('a'), 1);
        assert_eq!(get_item_priority('z'), 26);
        assert_eq!(get_item_priority('A'), 27);
        assert_eq!(get_item_priority('Z'), 52);
    }

    #[test]
    fn test_common_item_priorities_sum() {
        let rucksack_compartments = vec![
            (HashSet::from(['a']), HashSet::from(['a'])),
            (HashSet::from(['A', 'b']), HashSet::from(['B', 'A'])),
        ];
        assert_eq!(get_common_item_priorities_sum(&rucksack_compartments), 28);
    }

    #[test]
    fn test_elf_rucksacks_grouping() {
        let rucksack_compartments = vec![
            (HashSet::from(['a']), HashSet::from(['A'])),
            (HashSet::from(['b']), HashSet::from(['B'])),
            (HashSet::from(['c']), HashSet::from(['C'])),
            (HashSet::from(['d']), HashSet::from(['D'])),
            (HashSet::from(['e']), HashSet::from(['E'])),
            (HashSet::from(['f']), HashSet::from(['F'])),
        ];
        assert_eq!(
            group_elf_rucksacks(&rucksack_compartments),
            vec![
                vec![
                    HashSet::from(['a', 'A']),
                    HashSet::from(['b', 'B']),
                    HashSet::from(['c', 'C']),
                ],
                vec![
                    HashSet::from(['d', 'D']),
                    HashSet::from(['e', 'E']),
                    HashSet::from(['f', 'F']),
                ]
            ]
        )
    }
}
