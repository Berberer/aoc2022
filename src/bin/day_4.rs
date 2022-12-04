use std::collections::HashSet;

fn parse_input(input: &'static str) -> Vec<(HashSet<u32>, HashSet<u32>)> {
    input
        .lines()
        .map(|line| line.trim().split_once(',').unwrap())
        .map(|(sections_1, sections_2)| {
            (
                sections_1.split_once('-').unwrap(),
                sections_2.split_once('-').unwrap(),
            )
        })
        .map(
            |((section_1_start, section_1_end), (section_2_start, section_2_end))| {
                (
                    (
                        section_1_start.parse().unwrap(),
                        section_1_end.parse().unwrap(),
                    ),
                    (
                        section_2_start.parse().unwrap(),
                        section_2_end.parse().unwrap(),
                    ),
                )
            },
        )
        .map(
            |((section_1_start, section_1_end), (section_2_start, section_2_end))| {
                (
                    (section_1_start..=section_1_end).collect(),
                    (section_2_start..=section_2_end).collect(),
                )
            },
        )
        .collect()
}

fn count_fully_contained_sections(cleaning_section: &Vec<(HashSet<u32>, HashSet<u32>)>) -> usize {
    cleaning_section
        .iter()
        .filter(|(section_1, section_2)| {
            section_1.is_subset(section_2) || section_2.is_subset(section_1)
        })
        .count()
}

fn count_intersecting_sections(cleaning_section: &Vec<(HashSet<u32>, HashSet<u32>)>) -> usize {
    cleaning_section
        .iter()
        .filter(|(section_1, section_2)| !section_1.is_disjoint(section_2))
        .count()
}

fn main() {
    let input = include_str!("../inputs/data_day_4.txt");
    let cleaning_sections = parse_input(input);

    //Solution for puzzle 1
    let contained_sections = count_fully_contained_sections(&cleaning_sections);
    println!(
        "For {} cleaning assignments one section is contained in the other",
        contained_sections
    );

    //Solution for puzzle 2
    let intersecting_sections = count_intersecting_sections(&cleaning_sections);
    println!(
        "For {} cleaning assignments the two sections intersect",
        intersecting_sections
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_parsing() {
        let cleaning_sections = parse_input(
            "1-2,2-3
            4-6,5-7",
        );
        assert_eq!(
            cleaning_sections,
            vec![
                (HashSet::from([1, 2]), HashSet::from([2, 3])),
                (HashSet::from([4, 5, 6]), HashSet::from([5, 6, 7])),
            ]
        );
    }

    #[test]
    fn test_count_fully_contained_sections() {
        let cleaning_sections = vec![
            (HashSet::from([1, 2]), HashSet::from([3, 4])),
            (HashSet::from([5, 6, 7, 8]), HashSet::from([6, 7])),
            (HashSet::from([9, 10, 11]), HashSet::from([10])),
        ];
        assert_eq!(count_fully_contained_sections(&cleaning_sections), 2);
    }

    #[test]
    fn test_count_intersecting_sections() {
        let cleaning_sections = vec![
            (HashSet::from([1, 2]), HashSet::from([2, 3])),
            (HashSet::from([5, 6, 7, 8]), HashSet::from([6, 7])),
            (HashSet::from([9]), HashSet::from([10])),
        ];
        assert_eq!(count_intersecting_sections(&cleaning_sections), 2);
    }
}
