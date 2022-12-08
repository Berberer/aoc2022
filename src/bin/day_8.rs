fn parse_input(input: &'static str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|tree_height| tree_height.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn get_heights_from_the_west(y: usize, tree_heights: &Vec<Vec<u32>>) -> Vec<u32> {
    tree_heights[y].clone()
}

fn get_heights_from_the_east(y: usize, tree_heights: &Vec<Vec<u32>>) -> Vec<u32> {
    tree_heights[y].iter().cloned().rev().collect()
}

fn get_heights_from_the_north(x: usize, tree_heights: &Vec<Vec<u32>>) -> Vec<u32> {
    tree_heights
        .iter()
        .map(|horizontal_line| horizontal_line[x])
        .collect()
}

fn get_heights_from_the_south(x: usize, tree_heights: &Vec<Vec<u32>>) -> Vec<u32> {
    tree_heights
        .iter()
        .map(|horizontal_line| horizontal_line[x])
        .rev()
        .collect()
}

fn are_trees_in_line_visible(tree_heights: &Vec<u32>) -> Vec<bool> {
    let mut visible = vec![true; tree_heights.len()];
    let mut max_height = tree_heights[0];
    for i in 1..tree_heights.len() {
        let height = tree_heights[i];
        if height > max_height {
            max_height = height;
        } else {
            visible[i] = false;
        }
    }

    visible
}

fn get_visible_trees_from_the_west(tree_heights: &Vec<Vec<u32>>) -> Vec<Vec<bool>> {
    let mut visible = vec![vec![false; tree_heights[0].len()]; tree_heights.len()];
    for y in 0..tree_heights.len() {
        let tree_line_heights = get_heights_from_the_west(y, tree_heights);
        let hidden_from_the_west = are_trees_in_line_visible(&tree_line_heights);
        for (x, hidden) in hidden_from_the_west.iter().enumerate() {
            visible[y][x] = *hidden;
        }
    }
    visible
}

fn get_visible_trees_from_the_east(tree_heights: &Vec<Vec<u32>>) -> Vec<Vec<bool>> {
    let mut visible = vec![vec![false; tree_heights[0].len()]; tree_heights.len()];
    for y in 0..tree_heights.len() {
        let tree_line_heights = get_heights_from_the_east(y, tree_heights);
        let hidden_from_the_east = are_trees_in_line_visible(&tree_line_heights);
        for (x, hidden) in hidden_from_the_east.iter().rev().enumerate() {
            visible[y][x] = *hidden;
        }
    }
    visible
}

fn get_visible_trees_from_the_north(tree_heights: &Vec<Vec<u32>>) -> Vec<Vec<bool>> {
    let mut visible = vec![vec![false; tree_heights[0].len()]; tree_heights.len()];
    for y in 0..tree_heights.len() {
        let tree_line_heights = get_heights_from_the_north(y, tree_heights);
        let hidden_from_the_north = are_trees_in_line_visible(&tree_line_heights);
        for (x, hidden) in hidden_from_the_north.iter().enumerate() {
            visible[x][y] = *hidden;
        }
    }
    visible
}

fn get_visible_trees_from_the_south(tree_heights: &Vec<Vec<u32>>) -> Vec<Vec<bool>> {
    let mut visible = vec![vec![false; tree_heights[0].len()]; tree_heights.len()];
    for y in 0..tree_heights.len() {
        let tree_line_heights = get_heights_from_the_south(y, tree_heights);
        let hidden_from_the_south = are_trees_in_line_visible(&tree_line_heights);
        for (x, hidden) in hidden_from_the_south.iter().rev().enumerate() {
            visible[x][y] = *hidden;
        }
    }
    visible
}

fn get_tree_visibility(tree_heights: &Vec<Vec<u32>>) -> Vec<Vec<bool>> {
    let mut visible = vec![vec![false; tree_heights[0].len()]; tree_heights.len()];

    let visible_from_the_west = get_visible_trees_from_the_west(&tree_heights);
    let visible_from_the_east = get_visible_trees_from_the_east(&tree_heights);
    let visible_from_the_north = get_visible_trees_from_the_north(&tree_heights);
    let visible_from_the_south = get_visible_trees_from_the_south(&tree_heights);

    for y in 0..tree_heights.len() {
        for x in 0..tree_heights[0].len() {
            visible[y][x] = visible_from_the_west[y][x]
                || visible_from_the_east[y][x]
                || visible_from_the_north[y][x]
                || visible_from_the_south[y][x];
        }
    }

    visible
}

fn count_visible_trees(tree_visibility: &Vec<Vec<bool>>) -> u32 {
    let mut visible_trees = 0;

    for y in 0..tree_visibility.len() {
        for x in 0..tree_visibility[0].len() {
            if tree_visibility[y][x] {
                visible_trees += 1;
            }
        }
    }

    visible_trees
}

fn get_four_view_directions(
    x: usize,
    y: usize,
    tree_heights: &Vec<Vec<u32>>,
) -> (
    Option<Vec<u32>>,
    Option<Vec<u32>>,
    Option<Vec<u32>>,
    Option<Vec<u32>>,
) {
    let view_on_the_west_side = if x == 0 {
        None
    } else {
        Some(
            get_heights_from_the_west(y, tree_heights)[..x]
                .iter()
                .cloned()
                .rev()
                .collect(),
        )
    };
    let view_on_the_east_side = if x == tree_heights[0].len() - 1 {
        None
    } else {
        Some(get_heights_from_the_west(y, tree_heights)[x + 1..].to_vec())
    };
    let view_on_the_north_side = if y == 0 {
        None
    } else {
        Some(
            get_heights_from_the_north(x, tree_heights)[..y]
                .iter()
                .cloned()
                .rev()
                .collect(),
        )
    };
    let view_on_the_south_side = if y == tree_heights.len() - 1 {
        None
    } else {
        Some(get_heights_from_the_north(x, tree_heights)[y + 1..].to_vec())
    };
    (
        view_on_the_west_side,
        view_on_the_east_side,
        view_on_the_north_side,
        view_on_the_south_side,
    )
}

fn evaluate_view_direction(view_point_height: u32, heights_in_view_direction: Vec<u32>) -> u32 {
    let mut score = 0;
    for height in heights_in_view_direction {
        score += 1;
        if height >= view_point_height {
            break;
        }
    }
    score
}

fn calculate_scenic_score_of_tree(x: usize, y: usize, tree_heights: &Vec<Vec<u32>>) -> u32 {
    let tree_height = tree_heights[y][x];
    match get_four_view_directions(x, y, tree_heights) {
        (
            Some(west_side_view),
            Some(east_side_view),
            Some(north_side_view),
            Some(south_side_view),
        ) => {
            evaluate_view_direction(tree_height, west_side_view)
                * evaluate_view_direction(tree_height, east_side_view)
                * evaluate_view_direction(tree_height, north_side_view)
                * evaluate_view_direction(tree_height, south_side_view)
        }
        _ => 0,
    }
}

fn find_highest_scenic_score(tree_heights: &Vec<Vec<u32>>) -> u32 {
    let mut highest_scenic_score = 0;

    for y in 0..tree_heights.len() {
        for x in 0..tree_heights[0].len() {
            let scenic_score = calculate_scenic_score_of_tree(x, y, tree_heights);
            highest_scenic_score = highest_scenic_score.max(scenic_score);
        }
    }

    highest_scenic_score
}

fn main() {
    let input = include_str!("../inputs/data_day_8.txt");
    let tree_heights = parse_input(input);

    // Solution for puzzle 1
    let tree_visibility = get_tree_visibility(&tree_heights);
    let number_of_visible_trees = count_visible_trees(&tree_visibility);
    println!(
        "{} trees are visible from the outside",
        number_of_visible_trees
    );

    // Solution for puzzle 2
    let highest_scenic_score = find_highest_scenic_score(&tree_heights);
    println!(
        "{} is the highest scenic score of all trees",
        highest_scenic_score
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_parsing() {
        let tree_heights = parse_input("12\n34");
        assert_eq!(tree_heights, vec![vec![1, 2], vec![3, 4]]);
    }

    #[test]
    fn test_tree_heights_from_the_west() {
        let tree_heights = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(get_heights_from_the_west(0, &tree_heights), vec![1, 2, 3]);
        assert_eq!(get_heights_from_the_west(1, &tree_heights), vec![4, 5, 6]);
        assert_eq!(get_heights_from_the_west(2, &tree_heights), vec![7, 8, 9]);
    }

    #[test]
    fn test_tree_heights_from_the_east() {
        let tree_heights = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(get_heights_from_the_east(0, &tree_heights), vec![3, 2, 1]);
        assert_eq!(get_heights_from_the_east(1, &tree_heights), vec![6, 5, 4]);
        assert_eq!(get_heights_from_the_east(2, &tree_heights), vec![9, 8, 7]);
    }

    #[test]
    fn test_tree_heights_from_the_north() {
        let tree_heights = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(get_heights_from_the_north(0, &tree_heights), vec![1, 4, 7]);
        assert_eq!(get_heights_from_the_north(1, &tree_heights), vec![2, 5, 8]);
        assert_eq!(get_heights_from_the_north(2, &tree_heights), vec![3, 6, 9]);
    }

    #[test]
    fn test_tree_heights_from_the_south() {
        let tree_heights = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(get_heights_from_the_south(0, &tree_heights), vec![7, 4, 1]);
        assert_eq!(get_heights_from_the_south(1, &tree_heights), vec![8, 5, 2]);
        assert_eq!(get_heights_from_the_south(2, &tree_heights), vec![9, 6, 3]);
    }

    #[test]
    fn test_trees_in_line_visible() {
        let heights = vec![1, 2, 3, 9, 4, 5];
        assert_eq!(
            are_trees_in_line_visible(&heights),
            vec![true, true, true, true, false, false]
        );

        let heights = vec![9, 1, 2, 3, 4, 5];
        assert_eq!(
            are_trees_in_line_visible(&heights),
            vec![true, false, false, false, false, false]
        );

        let heights = vec![1, 1, 1, 1, 1, 1];
        assert_eq!(
            are_trees_in_line_visible(&heights),
            vec![true, false, false, false, false, false]
        );

        let heights = vec![1, 1, 1, 9, 1, 1];
        assert_eq!(
            are_trees_in_line_visible(&heights),
            vec![true, false, false, true, false, false]
        );

        let heights = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(
            are_trees_in_line_visible(&heights),
            vec![true, true, true, true, true, true]
        );
    }

    #[test]
    fn test_visible_trees_from_the_west() {
        let heights = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        let visible_from_the_west = get_visible_trees_from_the_west(&heights);
        assert_eq!(
            visible_from_the_west,
            vec![
                vec![true, false, false, true, false],
                vec![true, true, false, false, false],
                vec![true, false, false, false, false],
                vec![true, false, true, false, true],
                vec![true, true, false, true, false],
            ]
        );
    }

    #[test]
    fn test_visible_trees_from_the_east() {
        let heights = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        let visible_from_the_east = get_visible_trees_from_the_east(&heights);
        assert_eq!(
            visible_from_the_east,
            vec![
                vec![false, false, false, true, true],
                vec![false, false, true, false, true],
                vec![true, true, false, true, true],
                vec![false, false, false, false, true],
                vec![false, false, false, true, true],
            ]
        );
    }

    #[test]
    fn test_visible_trees_from_the_north() {
        let heights = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        let visible_from_the_north = get_visible_trees_from_the_north(&heights);
        assert_eq!(
            visible_from_the_north,
            vec![
                vec![true, true, true, true, true],
                vec![false, true, true, false, false],
                vec![true, false, false, false, false],
                vec![false, false, false, false, true],
                vec![false, false, false, true, false],
            ]
        );
    }

    #[test]
    fn test_visible_trees_from_the_south() {
        let heights = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        let visible_from_the_south = get_visible_trees_from_the_south(&heights);
        assert_eq!(
            visible_from_the_south,
            vec![
                vec![false, false, false, false, false],
                vec![false, false, false, false, false],
                vec![true, false, false, false, false],
                vec![false, false, true, false, true],
                vec![true, true, true, true, true],
            ]
        );
    }

    #[test]
    fn test_tree_visibility() {
        let heights = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        let visible = get_tree_visibility(&heights);
        assert_eq!(
            visible,
            vec![
                vec![true, true, true, true, true],
                vec![true, true, true, false, true],
                vec![true, true, false, true, true],
                vec![true, false, true, false, true],
                vec![true, true, true, true, true],
            ]
        );
    }

    #[test]
    fn test_count_visible_trees() {
        let tree_visibility = vec![
            vec![true, true, true],
            vec![true, false, true],
            vec![true, false, true],
            vec![true, true, true],
        ];
        assert_eq!(count_visible_trees(&tree_visibility), 10);
    }

    #[test]
    fn test_four_view_directions() {
        let heights = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        // North West corner point
        assert_eq!(
            get_four_view_directions(0, 0, &heights),
            (None, Some(vec![0, 3, 7, 3]), None, Some(vec![2, 6, 3, 3]))
        );
        // North East corner point
        assert_eq!(
            get_four_view_directions(4, 0, &heights),
            (Some(vec![7, 3, 0, 3]), None, None, Some(vec![2, 2, 9, 0]))
        );
        // South West corner point
        assert_eq!(
            get_four_view_directions(0, 4, &heights),
            (None, Some(vec![5, 3, 9, 0]), Some(vec![3, 6, 2, 3]), None)
        );
        // South East corner point
        assert_eq!(
            get_four_view_directions(4, 4, &heights),
            (Some(vec![9, 3, 5, 3]), None, Some(vec![9, 2, 2, 3]), None)
        );
        // Inner points
        assert_eq!(
            get_four_view_directions(1, 1, &heights),
            (
                Some(vec![2]),
                Some(vec![5, 1, 2]),
                Some(vec![0]),
                Some(vec![5, 3, 5])
            )
        );
        assert_eq!(
            get_four_view_directions(2, 2, &heights),
            (
                Some(vec![5, 6]),
                Some(vec![3, 2]),
                Some(vec![5, 3]),
                Some(vec![5, 3])
            )
        );
        assert_eq!(
            get_four_view_directions(3, 3, &heights),
            (
                Some(vec![5, 3, 3]),
                Some(vec![9]),
                Some(vec![3, 1, 7]),
                Some(vec![9])
            )
        );
    }

    #[test]
    fn test_evaluate_view_direction() {
        assert_eq!(
            evaluate_view_direction(5, vec![4, 3, 2, 1, 2, 3, 4, 5, 1]),
            8
        );
        assert_eq!(evaluate_view_direction(5, vec![5, 4, 3, 2, 1]), 1);
    }

    #[test]
    fn test_calculate_scenic_score_of_tree() {
        let heights = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(calculate_scenic_score_of_tree(0, 0, &heights), 0);
        assert_eq!(calculate_scenic_score_of_tree(2, 1, &heights), 4);
        assert_eq!(calculate_scenic_score_of_tree(2, 3, &heights), 8);
        assert_eq!(calculate_scenic_score_of_tree(4, 4, &heights), 0);
    }

    #[test]
    fn test_find_highest_scenic_score() {
        let heights = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(find_highest_scenic_score(&heights), 8);
    }
}
