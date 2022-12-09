use std::collections::HashSet;

fn parse_input(input: &'static str) -> Vec<(u32, (i32, i32))> {
    input
        .lines()
        .map(|line| line.trim().split_once(' ').unwrap())
        .map(|(direction, amount)| {
            (
                direction.chars().next().unwrap(),
                amount.parse::<u32>().unwrap(),
            )
        })
        .map(|(direction, amount)| match direction {
            'R' => (amount, (1, 0)),
            'L' => (amount, (-1, 0)),
            'U' => (amount, (0, 1)),
            _ => (amount, (0, -1)),
        })
        .collect()
}

fn get_next_rope_knot_position(
    current_rope_knot_position: &(i32, i32),
    rope_predecessor_position: &(i32, i32),
) -> (i32, i32) {
    let (current_tail_x, current_tail_y) = *current_rope_knot_position;
    let d_x = (rope_predecessor_position.0 - current_rope_knot_position.0) as f32;
    let d_y = (rope_predecessor_position.1 - current_rope_knot_position.1) as f32;
    let distance = (d_x * d_x + d_y * d_y).sqrt();
    if distance <= 2.0f32.sqrt() {
        (current_tail_x, current_tail_y)
    } else {
        let tail_move_x = if d_x < 0.0 {
            (d_x / distance).floor()
        } else {
            (d_x / distance).ceil()
        } as i32;
        let tail_move_y = if d_y < 0.0 {
            (d_y / distance).floor()
        } else {
            (d_y / distance).ceil()
        } as i32;

        (current_tail_x + tail_move_x, current_tail_y + tail_move_y)
    }
}

fn execute_rope_movement_step(
    movement_direction: &(i32, i32),
    rope_knot_positions: &[(i32, i32)],
) -> Vec<(i32, i32)> {
    let updated_rope_head_position = (
        rope_knot_positions[0].0 + movement_direction.0,
        rope_knot_positions[0].1 + movement_direction.1,
    );
    let mut updated_rope_knot_positions = vec![updated_rope_head_position];
    for (i, rope_knot_position) in rope_knot_positions.iter().skip(1).enumerate() {
        let updated_rope_knot_position =
            get_next_rope_knot_position(rope_knot_position, &updated_rope_knot_positions[i]);
        updated_rope_knot_positions.push(updated_rope_knot_position);
    }

    updated_rope_knot_positions
}

fn execute_rope_movement(
    rope_length: usize,
    moves: &Vec<(u32, (i32, i32))>,
) -> Vec<Vec<(i32, i32)>> {
    let mut current_rope_knot_positions = vec![(0, 0); rope_length];
    let mut rope_knot_paths = vec![vec![(0, 0)]; rope_length];

    for (amount, movement_direction) in moves {
        for _ in 0..*amount {
            current_rope_knot_positions =
                execute_rope_movement_step(movement_direction, &current_rope_knot_positions);
            for i in 0..rope_length {
                rope_knot_paths[i].push(current_rope_knot_positions[i]);
            }
        }
    }

    rope_knot_paths
}

fn get_unique_path_positions(path: &[(i32, i32)]) -> HashSet<(i32, i32)> {
    HashSet::from_iter(path.iter().cloned())
}

fn main() {
    let input = include_str!("../inputs/data_day_9.txt");
    let movements = parse_input(input);

    // Solution for puzzle 1
    let rope_knot_paths = execute_rope_movement(2, &movements);
    let tail_path = &rope_knot_paths[1];
    let unique_tail_positions = get_unique_path_positions(tail_path);
    println!(
        "During the movements of the rope with length 2, the tail is at {} different positions",
        unique_tail_positions.len()
    );

    // Solution for puzzle 2
    let rope_knot_paths = execute_rope_movement(10, &movements);
    let tail_path = &rope_knot_paths[9];
    let unique_tail_positions = get_unique_path_positions(tail_path);
    println!(
        "During the movements of the rope with length 10, the tail is at {} different positions",
        unique_tail_positions.len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_parsing() {
        let movements = parse_input(
            "L 1
            R 2
            U 3
            D 4",
        );
        assert_eq!(
            movements,
            vec![(1, (-1, 0)), (2, (1, 0)), (3, (0, 1)), (4, (0, -1))]
        );
    }

    #[test]
    fn test_next_tail_position() {
        // Tail touches head -> No movement
        assert_eq!(get_next_rope_knot_position(&(0, 0), &(0, 0)), (0, 0));
        assert_eq!(get_next_rope_knot_position(&(0, 0), &(1, 0)), (0, 0));
        assert_eq!(get_next_rope_knot_position(&(0, 0), &(-1, 0)), (0, 0));
        assert_eq!(get_next_rope_knot_position(&(0, 0), &(0, 1)), (0, 0));
        assert_eq!(get_next_rope_knot_position(&(0, 0), &(0, -1)), (0, 0));
        assert_eq!(get_next_rope_knot_position(&(0, 0), &(1, 1)), (0, 0));
        assert_eq!(get_next_rope_knot_position(&(0, 0), &(1, -1)), (0, 0));
        assert_eq!(get_next_rope_knot_position(&(0, 0), &(-1, 1)), (0, 0));
        assert_eq!(get_next_rope_knot_position(&(0, 0), &(-1, -1)), (0, 0));

        // Tail movement on straight line
        assert_eq!(get_next_rope_knot_position(&(0, 0), &(2, 0)), (1, 0));
        assert_eq!(get_next_rope_knot_position(&(0, 0), &(-2, 0)), (-1, 0));
        assert_eq!(get_next_rope_knot_position(&(0, 0), &(0, 2)), (0, 1));
        assert_eq!(get_next_rope_knot_position(&(0, 0), &(0, -2)), (0, -1));

        // Tail movement on diagonal line
        assert_eq!(get_next_rope_knot_position(&(0, 0), &(1, 2)), (1, 1));
        assert_eq!(get_next_rope_knot_position(&(0, 0), &(2, 1)), (1, 1));
        assert_eq!(get_next_rope_knot_position(&(0, 0), &(1, -2)), (1, -1));
        assert_eq!(get_next_rope_knot_position(&(0, 0), &(2, -1)), (1, -1));
        assert_eq!(get_next_rope_knot_position(&(0, 0), &(-1, 2)), (-1, 1));
        assert_eq!(get_next_rope_knot_position(&(0, 0), &(-2, 1)), (-1, 1));
        assert_eq!(get_next_rope_knot_position(&(0, 0), &(-1, -2)), (-1, -1));
        assert_eq!(get_next_rope_knot_position(&(0, 0), &(-2, -1)), (-1, -1));
    }

    #[test]
    fn test_execute_rope_movement() {
        let moves = vec![(1, (-1, 0)), (2, (1, 0)), (3, (0, 1)), (4, (0, -1))];
        let rope_knot_paths = execute_rope_movement(2, &moves);
        assert_eq!(
            rope_knot_paths[0],
            vec![
                (0, 0),
                (-1, 0),
                (0, 0),
                (1, 0),
                (1, 1),
                (1, 2),
                (1, 3),
                (1, 2),
                (1, 1),
                (1, 0),
                (1, -1),
            ]
        );
        assert_eq!(
            rope_knot_paths[1],
            vec![
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (0, 0),
                (1, 1),
                (1, 2),
                (1, 2),
                (1, 2),
                (1, 1),
                (1, 0),
            ]
        );
    }

    #[test]
    fn test_unique_path_positions() {
        let path = vec![
            (0, 0),
            (-1, 0),
            (0, 0),
            (1, 0),
            (1, 1),
            (1, 2),
            (1, 3),
            (1, 2),
            (1, 1),
            (1, 0),
            (1, -1),
        ];
        assert_eq!(
            get_unique_path_positions(&path),
            HashSet::from_iter(vec![
                (0, 0),
                (-1, 0),
                (1, 0),
                (1, 1),
                (1, 2),
                (1, 3),
                (1, -1),
            ])
        );
    }
}
