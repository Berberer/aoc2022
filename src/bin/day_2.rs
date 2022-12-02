fn parse_input(input: &'static str) -> Vec<(char, char)> {
    input
        .lines()
        .map(|line| line.trim().chars())
        .map(|mut chars| (chars.next().unwrap(), chars.nth(1).unwrap()))
        .collect()
}

fn opponent_shape_to_index(shape: &char) -> u32 {
    u32::from(*shape) - 64 // A is 65
}

fn player_shape_to_index(shape: &char) -> u32 {
    u32::from(*shape) - 87 // X is 88
}

fn evaluate_game(game_strategy: &(char, char)) -> u32 {
    let (opponent_shape, player_shape) = game_strategy;
    let opponent_shape = opponent_shape_to_index(opponent_shape);
    let player_shape = player_shape_to_index(player_shape);
    match (opponent_shape, player_shape) {
        // Victory Combinations
        (1, 2) | (2, 3) | (3, 1) => 6 + player_shape,
        // Draw
        (o, p) if o == p => 3 + p,
        // Defeat
        _ => player_shape,
    }
}

fn evaluate_strategy_guide(strategy_guide: &Vec<(char, char)>) -> u32 {
    strategy_guide.iter().map(evaluate_game).sum()
}

fn transform_game_outcome_instruction_to_strategy(
    game_outcome_instruction: &(char, char),
) -> (char, char) {
    let (opponent_shape, game_outcome) = game_outcome_instruction;
    let move_selection_index = (opponent_shape_to_index(opponent_shape) - 1) as usize;
    let player_shape = match game_outcome {
        // Victory
        'Z' => ['Y', 'Z', 'X'][move_selection_index],
        // Defeat
        'X' => ['Z', 'X', 'Y'][move_selection_index],
        // Draw
        _ => ['X', 'Y', 'Z'][move_selection_index],
    };
    (*opponent_shape, player_shape)
}

fn main() {
    let input = include_str!("../inputs/data_day_2.txt");
    let strategy_guide = parse_input(input);

    // Solution for puzzle 1
    let scores = evaluate_strategy_guide(&strategy_guide);
    println!(
        "Playing according to the strategy guide ends with {} points",
        scores
    );

    // Solution for puzzle 2
    let strategy_guide = strategy_guide
        .iter()
        .map(transform_game_outcome_instruction_to_strategy)
        .collect();
    let scores = evaluate_strategy_guide(&strategy_guide);
    println!(
        "Playing according to the game outcome instructions ends with {} points",
        scores
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_parsing() {
        let strategy_guide = parse_input(
            "A B
            C D",
        );
        assert_eq!(strategy_guide, vec![('A', 'B'), ('C', 'D')]);
    }

    #[test]
    fn test_opponent_shape_to_index() {
        assert_eq!(opponent_shape_to_index(&'A'), 1);
        assert_eq!(opponent_shape_to_index(&'B'), 2);
        assert_eq!(opponent_shape_to_index(&'C'), 3);
    }

    #[test]
    fn test_player_shape_to_index() {
        assert_eq!(player_shape_to_index(&'X'), 1);
        assert_eq!(player_shape_to_index(&'Y'), 2);
        assert_eq!(player_shape_to_index(&'Z'), 3);
    }

    #[test]
    fn test_game_evaluation_player_victory() {
        assert_eq!(evaluate_game(&('A', 'Y')), 8);
        assert_eq!(evaluate_game(&('B', 'Z')), 9);
        assert_eq!(evaluate_game(&('C', 'X')), 7);
    }

    #[test]
    fn test_game_evaluation_player_defeat() {
        assert_eq!(evaluate_game(&('A', 'Z')), 3);
        assert_eq!(evaluate_game(&('B', 'X')), 1);
        assert_eq!(evaluate_game(&('C', 'Y')), 2);
    }

    #[test]
    fn test_game_evaluation_draw() {
        assert_eq!(evaluate_game(&('A', 'X')), 4);
        assert_eq!(evaluate_game(&('B', 'Y')), 5);
        assert_eq!(evaluate_game(&('C', 'Z')), 6);
    }

    #[test]
    fn test_strategy_guide_evaluation() {
        let strategy_guide = vec![('A', 'Y'), ('A', 'Z'), ('A', 'X')];
        let score = evaluate_strategy_guide(&strategy_guide);
        assert_eq!(score, 15);
    }

    #[test]
    fn test_transform_game_outcome_instruction_to_victory_strategy() {
        assert_eq!(
            transform_game_outcome_instruction_to_strategy(&('A', 'Z')),
            ('A', 'Y')
        );
        assert_eq!(
            transform_game_outcome_instruction_to_strategy(&('B', 'Z')),
            ('B', 'Z')
        );
        assert_eq!(
            transform_game_outcome_instruction_to_strategy(&('C', 'Z')),
            ('C', 'X')
        );
    }

    #[test]
    fn test_transform_game_outcome_instruction_to_defeat_strategy() {
        assert_eq!(
            transform_game_outcome_instruction_to_strategy(&('A', 'X')),
            ('A', 'Z')
        );
        assert_eq!(
            transform_game_outcome_instruction_to_strategy(&('B', 'X')),
            ('B', 'X')
        );
        assert_eq!(
            transform_game_outcome_instruction_to_strategy(&('C', 'X')),
            ('C', 'Y')
        );
    }

    #[test]
    fn test_transform_game_outcome_instruction_to_draw_strategy() {
        assert_eq!(
            transform_game_outcome_instruction_to_strategy(&('A', 'Y')),
            ('A', 'X')
        );
        assert_eq!(
            transform_game_outcome_instruction_to_strategy(&('B', 'Y')),
            ('B', 'Y')
        );
        assert_eq!(
            transform_game_outcome_instruction_to_strategy(&('C', 'Y')),
            ('C', 'Z')
        );
    }
}
