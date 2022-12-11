use std::collections::HashMap;

enum WorryLevelReduction {
    Divide(u64),
    Modulo(u64),
}

struct Monkey {
    item_worry_levels: Vec<u64>,
    inspections_counter: u64,
    worry_level_change_operation: Box<dyn Fn(u64) -> u64>,
    worry_level_test_parameter: u64,
    worry_level_test_successful_monkey_index: usize,
    worry_level_test_unsuccessful_monkey_index: usize,
}

impl Monkey {
    fn new(input: &'static str) -> Self {
        let mut input_lines = input.lines().skip(1);
        let item_worry_levels = parse_item_worry_levels(input_lines.next().unwrap());
        let worry_level_change_operation =
            parse_worry_level_change_operation(input_lines.next().unwrap());
        let worry_level_test_parameter =
            parse_worry_level_test_parameter(input_lines.next().unwrap());
        let worry_level_test_successful_monkey_index =
            parse_monkey_throw_index(input_lines.next().unwrap());
        let worry_level_test_unsuccessful_monkey_index =
            parse_monkey_throw_index(input_lines.next().unwrap());
        Self {
            inspections_counter: 0,
            item_worry_levels,
            worry_level_change_operation,
            worry_level_test_parameter,
            worry_level_test_successful_monkey_index,
            worry_level_test_unsuccessful_monkey_index,
        }
    }

    fn execute_turn(
        &mut self,
        new_item_worry_levels: &[u64],
        worry_level_reduction: &WorryLevelReduction,
    ) -> HashMap<usize, Vec<u64>> {
        let item_worry_levels = self
            .item_worry_levels
            .iter()
            .cloned()
            .chain(new_item_worry_levels.iter().cloned())
            .collect::<Vec<u64>>();

        self.item_worry_levels.clear();

        let mut item_throws: HashMap<usize, Vec<u64>> = HashMap::new();

        for item_worry_level in item_worry_levels {
            self.inspections_counter += 1;

            let changed_item_worry_level: u64 =
                (self.worry_level_change_operation)(item_worry_level);

            let changed_item_worry_level = match worry_level_reduction {
                WorryLevelReduction::Divide(n) => changed_item_worry_level / n,
                WorryLevelReduction::Modulo(n) => changed_item_worry_level % n,
            };

            let monkey_throw_index =
                if changed_item_worry_level % self.worry_level_test_parameter == 0 {
                    self.worry_level_test_successful_monkey_index
                } else {
                    self.worry_level_test_unsuccessful_monkey_index
                };

            insert_item_throw(
                monkey_throw_index,
                vec![changed_item_worry_level],
                &mut item_throws,
            );
        }

        item_throws
    }
}

fn parse_item_worry_levels(input: &'static str) -> Vec<u64> {
    input
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split(", ")
        .map(|l| l.parse().unwrap())
        .collect()
}

fn parse_worry_level_change_operation(input: &'static str) -> Box<dyn Fn(u64) -> u64> {
    let operation_calculation_tokens = input
        .split_once('=')
        .unwrap()
        .1
        .split_whitespace()
        .collect::<Vec<&str>>();

    let mut constants = Vec::new();
    if operation_calculation_tokens[0] != "old" {
        constants.push(operation_calculation_tokens[0].parse::<u64>().unwrap());
    }
    if operation_calculation_tokens[2] != "old" {
        constants.push(operation_calculation_tokens[2].parse::<u64>().unwrap())
    }

    let input_parameter_usage_counter = 2 - constants.len();

    if operation_calculation_tokens[1] == "+" {
        Box::new(move |parameter| {
            let mut calculation_inputs = vec![parameter; input_parameter_usage_counter];
            calculation_inputs.extend(constants.iter());
            calculation_inputs.iter().sum()
        })
    } else {
        Box::new(move |parameter| {
            let mut calculation_inputs = vec![parameter; input_parameter_usage_counter];
            calculation_inputs.extend(constants.iter());
            calculation_inputs.iter().product()
        })
    }
}

fn parse_worry_level_test_parameter(input: &'static str) -> u64 {
    input.split_once("by").unwrap().1.trim().parse().unwrap()
}

fn parse_monkey_throw_index(input: &'static str) -> usize {
    input
        .split_once("monkey")
        .unwrap()
        .1
        .trim()
        .parse()
        .unwrap()
}

fn parse_input(input: &'static str) -> Vec<Monkey> {
    input.split("\n\n").map(Monkey::new).collect()
}

fn insert_item_throw(
    index: usize,
    thrown_items: Vec<u64>,
    item_throws: &mut HashMap<usize, Vec<u64>>,
) {
    if let Some(monkey_item_throws) = item_throws.get_mut(&index) {
        monkey_item_throws.extend(thrown_items);
    } else {
        item_throws.insert(index, thrown_items);
    }
}

fn execute_monkey_throwing_round(
    monkeys: &mut [Monkey],
    worry_level_reduction: WorryLevelReduction,
) {
    let mut item_throws: HashMap<usize, Vec<u64>> = HashMap::new();

    for (index, monkey) in monkeys.iter_mut().enumerate() {
        let items_thrown_to_monkey = item_throws.remove(&index).unwrap_or_default();
        let items_thrown_from_monkey =
            monkey.execute_turn(&items_thrown_to_monkey, &worry_level_reduction);
        for (monkey_index, thrown_items) in items_thrown_from_monkey {
            insert_item_throw(monkey_index, thrown_items, &mut item_throws);
        }
    }

    for (monkey_index, thrown_items) in item_throws {
        monkeys[monkey_index].item_worry_levels.extend(thrown_items);
    }
}

fn main() {
    let input = include_str!("../inputs/data_day_11.txt");

    // Solution for puzzle 1
    let mut monkeys = parse_input(input);
    for _ in 0..20 {
        execute_monkey_throwing_round(&mut monkeys, WorryLevelReduction::Divide(3));
    }
    let mut inspections = monkeys
        .iter()
        .map(|m| m.inspections_counter)
        .collect::<Vec<u64>>();
    inspections.sort();
    inspections.reverse();
    println!(
        "The product of the two highest monkey inspection counters with worry level reduction is {}",
        inspections[0] * inspections[1]
    );

    //Solution for puzzle 2
    let mut monkeys = parse_input(input);
    let worry_level_reduction_factor = monkeys
        .iter()
        .map(|m| m.worry_level_test_parameter)
        .product();
    for _ in 0..10000 {
        execute_monkey_throwing_round(
            &mut monkeys,
            WorryLevelReduction::Modulo(worry_level_reduction_factor),
        );
    }
    let mut inspections = monkeys
        .iter()
        .map(|m| m.inspections_counter)
        .collect::<Vec<u64>>();
    inspections.sort();
    inspections.reverse();
    println!(
        "The product of the two highest monkey inspection counters without worry level reduction is {}",
        inspections[0] * inspections[1]
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_item_worry_levels() {
        assert_eq!(parse_item_worry_levels("Starting items: 123"), vec![123]);
        assert_eq!(
            parse_item_worry_levels("Starting items: 1, 22, 333"),
            vec![1, 22, 333]
        );
    }

    #[test]
    fn test_parse_worry_level_change_operation() {
        let tests = vec![
            ("Operation: new = old + 2", (5u64, 7)),
            ("Operation: new = 2 + old", (5u64, 7)),
            ("Operation: new = old + old", (5u64, 10)),
            ("Operation: new = 2 + 3", (5u64, 5)),
            ("Operation: new = old * 2", (5u64, 10)),
            ("Operation: new = 2 * old", (5u64, 10)),
            ("Operation: new = old * old", (5u64, 25)),
            ("Operation: new = 2 * 3", (5u64, 6)),
        ];

        for (operation, (parameter, correct_result)) in tests {
            let operation_function = parse_worry_level_change_operation(operation);
            assert_eq!(operation_function(parameter), correct_result);
        }
    }

    #[test]
    fn test_parse_worry_level_test_parameter() {
        assert_eq!(
            parse_worry_level_test_parameter("Test: divisible by 123"),
            123
        );
    }

    #[test]
    fn test_parse_monkey_throw_index() {
        assert_eq!(
            parse_monkey_throw_index("If true: throw to monkey 123"),
            123
        );
        assert_eq!(
            parse_monkey_throw_index("If false: throw to monkey 456"),
            456
        );
    }

    #[test]
    fn test_monkey_creation() {
        let input = "Monkey 1:
          Starting items: 1, 2, 3
          Operation: new = old * 2
          Test: divisible by 8
            If true: throw to monkey 2
            If false: throw to monkey 3";
        let monkey = Monkey::new(input);
        assert_eq!(monkey.item_worry_levels, vec![1, 2, 3]);
        assert_eq!(monkey.inspections_counter, 0);
        assert_eq!((monkey.worry_level_change_operation)(3), 6);
        assert_eq!(monkey.worry_level_test_parameter, 8);
        assert_eq!(monkey.worry_level_test_successful_monkey_index, 2);
        assert_eq!(monkey.worry_level_test_unsuccessful_monkey_index, 3);
    }

    #[test]
    fn test_input_parsing() {
        let input = "Monkey 1:
          Starting items: 1, 2, 3
          Operation: new = old * 2
          Test: divisible by 8
            If true: throw to monkey 2
            If false: throw to monkey 3

        Monkey 2:
          Starting items: 4, 5, 6
          Operation: new = old * 2
          Test: divisible by 8
            If true: throw to monkey 2
            If false: throw to monkey 3";

        let monkeys = parse_input(input);
        assert_eq!(monkeys.len(), 2);
        assert_eq!(monkeys[0].item_worry_levels, vec![1, 2, 3]);
        assert_eq!(monkeys[1].item_worry_levels, vec![4, 5, 6]);
    }

    #[test]
    fn test_monkey_turn_execution() {
        let mut monkey = Monkey {
            item_worry_levels: vec![1],
            inspections_counter: 0,
            worry_level_change_operation: Box::new(|worry_level| worry_level + 10),
            worry_level_test_parameter: 3,
            worry_level_test_successful_monkey_index: 1,
            worry_level_test_unsuccessful_monkey_index: 2,
        };
        let item_throws = monkey.execute_turn(&[2], &WorryLevelReduction::Divide(3));
        assert_eq!(monkey.inspections_counter, 2);
        assert!(monkey.item_worry_levels.is_empty());
        assert_eq!(item_throws, HashMap::from([(1, vec![3]), (2, vec![4])]));
    }

    #[test]
    fn test_monkey_round_execution() {
        let mut monkeys = vec![
            Monkey {
                item_worry_levels: vec![1],
                inspections_counter: 0,
                worry_level_change_operation: Box::new(|worry_level| worry_level * 3),
                worry_level_test_parameter: 1,
                worry_level_test_successful_monkey_index: 1,
                worry_level_test_unsuccessful_monkey_index: 1,
            },
            Monkey {
                item_worry_levels: vec![2],
                inspections_counter: 0,
                worry_level_change_operation: Box::new(|worry_level| worry_level * 3),
                worry_level_test_parameter: 1,
                worry_level_test_successful_monkey_index: 0,
                worry_level_test_unsuccessful_monkey_index: 0,
            },
        ];
        execute_monkey_throwing_round(&mut monkeys, WorryLevelReduction::Divide(3));
        assert_eq!(monkeys[0].inspections_counter, 1);
        assert_eq!(monkeys[0].item_worry_levels, vec![2, 1]);
        assert_eq!(monkeys[1].inspections_counter, 2);
        assert!(monkeys[1].item_worry_levels.is_empty());
    }
}
