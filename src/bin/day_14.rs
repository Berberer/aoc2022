fn parse_input(input: &'static str) -> Vec<Vec<(usize, usize)>> {
    let mut paths = Vec::new();

    for path in input.lines() {
        let mut path_points = Vec::new();
        for path_point in path.trim().split(" -> ") {
            let (x, y) = path_point.split_once(',').unwrap();
            let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
            path_points.push((x, y));
        }
        paths.push(path_points)
    }

    paths
}

fn create_cave_system(paths: Vec<Vec<(usize, usize)>>) -> Vec<Vec<bool>> {
    let height = paths
        .iter()
        .map(|path_points| path_points.iter().map(|p| p.1).max().unwrap())
        .max()
        .unwrap();

    let mut cave_system = vec![vec![false; 1000]; height + 1];

    for path in paths {
        for path_segment in path.windows(2) {
            let x_1 = path_segment[0].0;
            let x_2 = path_segment[1].0;
            let x_start = x_1.min(x_2);
            let x_end = x_1.max(x_2);
            for x in x_start..=x_end {
                let y_1 = path_segment[0].1;
                let y_2 = path_segment[1].1;
                let y_start = y_1.min(y_2);
                let y_end = y_1.max(y_2);
                for y in y_start..=y_end {
                    cave_system[y][x] = true;
                }
            }
        }
    }

    cave_system
}

fn get_final_sand_position(
    sand_source_index: usize,
    cave_spec: &Vec<Vec<bool>>,
) -> Option<(usize, usize)> {
    let mut sand_x = sand_source_index;
    let mut sand_y = 0;
    let mut final_position_reached = false;

    while !final_position_reached {
        if !cave_spec[sand_y + 1][sand_x] {
            sand_y += 1;
        } else if !cave_spec[sand_y + 1][sand_x - 1] {
            sand_y += 1;
            sand_x -= 1;
        } else if !cave_spec[sand_y + 1][sand_x + 1] {
            sand_y += 1;
            sand_x += 1;
        } else {
            final_position_reached = true;
        }

        if sand_y + 1 == cave_spec.len() {
            return None;
        }
    }

    Some((sand_x, sand_y))
}

fn fill_with_sand(sand_source_index: usize, cave_system: &Vec<Vec<bool>>) -> usize {
    let mut sand_counter = 0;
    let mut cave_system = cave_system.clone();

    while let Some((x, y)) = get_final_sand_position(sand_source_index, &cave_system) {
        sand_counter += 1;
        cave_system[y][x] = true;
        if x == sand_source_index && y == 0 {
            break;
        }
    }

    sand_counter
}

fn main() {
    let input = include_str!("../inputs/data_day_14.txt");
    let cave_spec = parse_input(input);
    let cave_system = create_cave_system(cave_spec);

    //Solution for puzzle 1
    let sand_amount_in_filled_cave = fill_with_sand(500, &cave_system);
    println!(
        "The cave system can be filled with {} units of sand before sand falls into the abyss",
        sand_amount_in_filled_cave
    );

    // Solution for puzzle 2
    let mut cave_with_floor = cave_system.clone();
    cave_with_floor.push(vec![false; 1000]);
    cave_with_floor.push(vec![true; 1000]);
    let sand_amount_in_filled_cave = fill_with_sand(500, &cave_with_floor);
    println!(
        "The cave system can be filled with {} units of sand before the sand source is blocked",
        sand_amount_in_filled_cave
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_parsing() {
        let paths = parse_input("0,1 -> 2,3 -> 4,5\n6,7 -> 8,9");
        assert_eq!(
            paths,
            vec![vec![(0, 1), (2, 3), (4, 5)], vec![(6, 7), (8, 9)]]
        );
    }

    #[test]
    fn test_create_cave_system() {
        let paths = vec![vec![(499, 3), (501, 3), (501, 1)], vec![(500, 1), (500, 3)]];
        let cave_system = create_cave_system(paths);

        assert!(cave_system[3][499]);
        assert!(cave_system[3][500]);
        assert!(cave_system[3][501]);
        assert!(cave_system[2][501]);
        assert!(cave_system[1][501]);
        assert!(cave_system[1][500]);
        assert!(cave_system[2][500]);
    }

    #[test]
    fn test_final_sand_position() {
        let cave_system = vec![
            vec![false, false, false, false],
            vec![true, false, false, false],
            vec![true, true, false, false],
            vec![false, false, false, false],
            vec![false, false, false, false],
            vec![false, false, false, true],
            vec![false, false, true, true],
            vec![true, true, true, true],
        ];
        let sand_position = get_final_sand_position(2, &cave_system).unwrap();
        assert_eq!(sand_position, (1, 6));
    }

    #[test]
    fn test_fill_with_sand() {
        let cave_system = vec![
            vec![false, false, false, false, true],
            vec![false, false, false, false, true],
            vec![false, false, false, false, true],
            vec![false, true, true, true, true],
        ];
        assert_eq!(fill_with_sand(3, &cave_system), 3);
    }
}
