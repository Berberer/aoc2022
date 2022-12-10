#[derive(Debug, PartialEq)]
enum Instruction {
    NoOp,
    AddX(i32),
}

impl Instruction {
    fn new(instruction_line: &'static str) -> Self {
        let mut tokens = instruction_line.split_whitespace();
        let instruction_keyword = tokens.next().unwrap();
        if instruction_keyword == "addx" {
            Self::AddX(tokens.next().unwrap().parse().unwrap())
        } else {
            Self::NoOp
        }
    }

    fn execute_instruction(&self, register_x: i32) -> Vec<(i32, i32)> {
        match self {
            Instruction::NoOp => vec![(register_x, register_x)],
            Instruction::AddX(n) => vec![(register_x, register_x), (register_x, register_x + n)],
        }
    }
}

fn parse_input(input: &'static str) -> Vec<Instruction> {
    input.lines().map(Instruction::new).collect()
}

fn execute_program(register_x: i32, program_instructions: &Vec<Instruction>) -> Vec<(i32, i32)> {
    let mut register_values_during_program = vec![(register_x, register_x)];
    for instruction in program_instructions {
        register_values_during_program.extend(
            instruction.execute_instruction(register_values_during_program.last().unwrap().1),
        );
    }

    register_values_during_program
}

fn get_signal_strengths_during_cycles(
    cycles: Vec<usize>,
    register_values: &[(i32, i32)],
) -> Vec<i32> {
    cycles
        .iter()
        .cloned()
        .map(|cycle| register_values[cycle].0 * cycle as i32)
        .collect()
}

fn draw_crt_line(register_values: &[(i32, i32)]) -> String {
    register_values
        .iter()
        .map(|(value_during_cycle, _)| (value_during_cycle - 1)..=(value_during_cycle + 1))
        .enumerate()
        .map(|(crt_position, sprite_pixel_positions)| {
            if sprite_pixel_positions.contains(&(crt_position as i32)) {
                '#'
            } else {
                '.'
            }
        })
        .collect()
}

fn main() {
    let input = include_str!("../inputs/data_day_10.txt");
    let program_instructions = parse_input(input);

    // Solution for puzzle 1
    let register_values_during_program = execute_program(1, &program_instructions);
    let signal_strengths_during_cycles = get_signal_strengths_during_cycles(
        vec![20, 60, 100, 140, 180, 220],
        &register_values_during_program,
    );
    let signal_strengths_sum = signal_strengths_during_cycles.iter().sum::<i32>();
    println!("Sum of signal strengths during the 20th, 60th, 100th, 140th, 180th, and 220th cycles is {}", signal_strengths_sum);

    // Solution for puzzle 2
    let crt_lines = register_values_during_program[1..]
        .chunks(40)
        .map(draw_crt_line)
        .collect::<Vec<String>>();
    println!("The program draws the following image on the CRT screen:");
    for crt_line in crt_lines {
        println!("{}", crt_line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_creation() {
        assert_eq!(Instruction::new("noop"), Instruction::NoOp);
        assert_eq!(Instruction::new("addx 123"), Instruction::AddX(123));
        assert_eq!(Instruction::new("addx 0"), Instruction::AddX(0));
        assert_eq!(Instruction::new("addx -456"), Instruction::AddX(-456));
    }

    #[test]
    fn test_input_parsing() {
        let input = "addx 1
        noop
        addx -1
        noop";
        let instructions = parse_input(input);
        assert_eq!(
            instructions,
            vec![
                Instruction::AddX(1),
                Instruction::NoOp,
                Instruction::AddX(-1),
                Instruction::NoOp,
            ]
        );
    }

    #[test]
    fn test_program_execution() {
        let program_instructions = vec![
            Instruction::AddX(1),
            Instruction::NoOp,
            Instruction::AddX(-1),
            Instruction::NoOp,
        ];
        let register_values_during_program = execute_program(0, &program_instructions);
        assert_eq!(
            register_values_during_program,
            vec![(0, 0), (0, 0), (0, 1), (1, 1), (1, 1), (1, 0), (0, 0)]
        );
    }

    #[test]
    fn test_signal_strengths_during_cycles() {
        let register_values_during_program = vec![(0, 0), (0, 0), (0, 1), (1, 1), (1, 2), (2, 2)];
        let signal_strengths_during_cycles =
            get_signal_strengths_during_cycles(vec![1, 3, 5], &register_values_during_program);
        assert_eq!(signal_strengths_during_cycles, vec![0, 3, 10]);
    }

    #[test]
    fn test_draw_crt_line() {
        let register_values = vec![(1, 1), (1, 16), (16, 16), (16, -11)];
        assert_eq!(draw_crt_line(&register_values), String::from("##.."))
    }
}
