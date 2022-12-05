struct CargoStacks {
    stacks: Vec<Vec<char>>,
}

impl CargoStacks {
    fn new(initial_stacks: &'static str) -> Self {
        let initial_stacks_lines = initial_stacks.lines().collect::<Vec<&str>>();
        let number_of_lines = initial_stacks_lines.len();
        let initial_stack_composition = initial_stacks_lines.iter().take(number_of_lines - 1);
        let stacks_number = initial_stacks_lines
            .last()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let mut stacks = vec![Vec::new(); stacks_number];

        for stack_layer in initial_stack_composition {
            stack_layer
                .chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .enumerate()
                .filter(|(_, cargo_crate)| cargo_crate[1].is_ascii_alphabetic())
                .for_each(|(stack_index, cargo_crate)| {
                    stacks[stack_index].insert(0, cargo_crate[1])
                });
        }

        Self { stacks }
    }

    fn execute_movement_command_single_crate(&mut self, command: &MovementCommand) {
        for _ in 0..command.amount {
            let cargo_crate = self.stacks[command.from - 1].pop().unwrap();
            self.stacks[command.to - 1].push(cargo_crate);
        }
    }

    fn execute_movement_command_multi_crate(&mut self, command: &MovementCommand) {
        let source_stack = self.stacks[command.from - 1].clone();
        self.stacks[command.to - 1]
            .extend_from_slice(&source_stack[(source_stack.len() - command.amount)..]);
        self.stacks[command.from - 1].truncate(source_stack.len() - command.amount);
    }

    fn get_top_crates(&self) -> Vec<char> {
        self.stacks
            .iter()
            .filter_map(|stack| stack.last())
            .cloned()
            .collect()
    }
}

struct MovementCommand {
    from: usize,
    to: usize,
    amount: usize,
}

impl MovementCommand {
    fn new(command_line: &'static str) -> Self {
        let mut command_parts = command_line.split_whitespace();
        let amount = command_parts.nth(1).unwrap().parse().unwrap();
        let from = command_parts.nth(1).unwrap().parse().unwrap();
        let to = command_parts.nth(1).unwrap().parse().unwrap();

        Self { from, to, amount }
    }
}

fn parse_input(input: &'static str) -> (CargoStacks, Vec<MovementCommand>) {
    let (initial_stack_composition, movement_commands) = input.split_once("\n\n").unwrap();
    let initial_stack_composition = CargoStacks::new(initial_stack_composition);
    let movement_commands = movement_commands
        .lines()
        .map(MovementCommand::new)
        .collect();
    (initial_stack_composition, movement_commands)
}

fn main() {
    let input = include_str!("../inputs/data_day_5.txt");

    // Solution for puzzle 1
    let (mut stack_composition, movement_commands) = parse_input(input);
    movement_commands
        .iter()
        .for_each(|command| stack_composition.execute_movement_command_single_crate(command));
    let top_cargo_crates = stack_composition
        .get_top_crates()
        .iter()
        .collect::<String>();
    println!(
        "The top cargo crates of all stacks after single crate movements are {}",
        top_cargo_crates
    );

    // Solution for puzzle 2
    let (mut stack_composition, movement_commands) = parse_input(input);
    movement_commands
        .iter()
        .for_each(|command| stack_composition.execute_movement_command_multi_crate(command));
    let top_cargo_crates = stack_composition
        .get_top_crates()
        .iter()
        .collect::<String>();
    println!(
        "The top cargo crates of all stacks after multi crate movement are {}",
        top_cargo_crates
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movement_command_from_command_text() {
        let movement_command = MovementCommand::new("move 1 from 22 to 333");
        assert_eq!(movement_command.amount, 1);
        assert_eq!(movement_command.from, 22);
        assert_eq!(movement_command.to, 333);
    }

    #[test]
    fn test_cargo_stack_from_text() {
        let stack_configuration = "    [A]    \n[B] [C] [D]\n 1   2   3 ";
        let cargo_stacks = CargoStacks::new(stack_configuration);
        assert_eq!(
            cargo_stacks.stacks,
            vec![vec!['B'], vec!['C', 'A'], vec!['D']]
        )
    }

    #[test]
    fn test_input_parsing() {
        let input =
            "    [A]    \n[B] [C] [D]\n 1   2   3 \n\nmove 1 from 2 to 3\nmove 4 from 5 to 6";
        let (initial_stack_composition, movement_commands) = parse_input(input);
        assert_eq!(
            initial_stack_composition.stacks,
            vec![vec!['B'], vec!['C', 'A'], vec!['D']]
        );
        assert_eq!(movement_commands[0].amount, 1);
        assert_eq!(movement_commands[0].from, 2);
        assert_eq!(movement_commands[0].to, 3);
        assert_eq!(movement_commands[1].amount, 4);
        assert_eq!(movement_commands[1].from, 5);
        assert_eq!(movement_commands[1].to, 6);
    }

    #[test]
    fn test_movement_command_single_crate_execution() {
        let mut cargo_stacks = CargoStacks {
            stacks: vec![vec!['B'], vec!['C', 'A'], vec!['D']],
        };
        let single_crate_command = MovementCommand {
            amount: 1,
            from: 1,
            to: 3,
        };
        cargo_stacks.execute_movement_command_single_crate(&single_crate_command);
        assert_eq!(
            cargo_stacks.stacks,
            vec![vec![], vec!['C', 'A'], vec!['D', 'B']]
        );
        let multi_crate_command = MovementCommand {
            amount: 2,
            from: 2,
            to: 3,
        };
        cargo_stacks.execute_movement_command_single_crate(&multi_crate_command);
        assert_eq!(
            cargo_stacks.stacks,
            vec![vec![], vec![], vec!['D', 'B', 'A', 'C']]
        );
    }

    #[test]
    fn test_movement_command_multi_crate_execution() {
        let mut cargo_stacks = CargoStacks {
            stacks: vec![vec!['B'], vec!['C', 'A'], vec!['D']],
        };
        let single_crate_command = MovementCommand {
            amount: 1,
            from: 1,
            to: 3,
        };
        cargo_stacks.execute_movement_command_multi_crate(&single_crate_command);
        assert_eq!(
            cargo_stacks.stacks,
            vec![vec![], vec!['C', 'A'], vec!['D', 'B']]
        );
        let multi_crate_command = MovementCommand {
            amount: 2,
            from: 2,
            to: 3,
        };
        cargo_stacks.execute_movement_command_multi_crate(&multi_crate_command);
        assert_eq!(
            cargo_stacks.stacks,
            vec![vec![], vec![], vec!['D', 'B', 'C', 'A']]
        );
    }

    #[test]
    fn test_get_top_crates() {
        let cargo_stacks = CargoStacks {
            stacks: vec![vec!['B'], vec!['C', 'A'], vec!['D']],
        };
        assert_eq!(cargo_stacks.get_top_crates(), vec!['B', 'A', 'D']);
    }
}
