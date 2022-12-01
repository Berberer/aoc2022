fn sum_inventory_lines(lines: &Vec<&'static str>) -> u32 {
    lines.iter().map(|line| line.parse::<u32>().unwrap()).sum()
}

fn parse_input(input: &'static str) -> Vec<u32> {
    let mut elf_inventories = Vec::new();
    let mut current_inventory_lines = Vec::new();
    for line in input.lines() {
        if line.trim().is_empty() {
            elf_inventories.push(sum_inventory_lines(&current_inventory_lines));
            current_inventory_lines.clear();
        } else {
            current_inventory_lines.push(line.trim());
        }
    }
    elf_inventories.push(sum_inventory_lines(&current_inventory_lines));

    elf_inventories
}

fn sum_first_n(n: usize, elf_inventories: &Vec<u32>) -> u32 {
    elf_inventories[0..n].iter().sum()
}

fn main() {
    let input = include_str!("../inputs/data_day_1.txt");
    let mut elf_inventories = parse_input(input);
    elf_inventories.sort();
    elf_inventories.reverse();

    // Solution for puzzle 1
    let max_calories_inventory = sum_first_n(1, &elf_inventories);
    println!(
        "Inventory with max calories has {} calories",
        max_calories_inventory
    );

    // Solution for puzzle 2
    let max_calories_inventory = sum_first_n(3, &elf_inventories);
    println!(
        "Inventory with max calories has {} calories",
        max_calories_inventory
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elf_inventory_creation() {
        let elf_inventory = sum_inventory_lines(&vec!["100", "200", "1"]);
        assert_eq!(elf_inventory, 301);
    }

    #[test]
    fn test_input_parsing() {
        let input = "200
        100

        50";
        let elf_inventories = parse_input(input);
        assert_eq!(elf_inventories, vec![300, 50]);
    }

    #[test]
    fn test_first_calories_sum() {
        let inventories = vec![2, 1, 4, 3];
        assert_eq!(sum_first_n(2, &inventories), 3);
    }
}
