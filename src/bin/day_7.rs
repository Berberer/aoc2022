use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
enum ShellCommand {
    Cd(CdDirection),
    Ls(Vec<DirChild>),
}

#[derive(Debug, Eq, PartialEq)]
enum CdDirection {
    Root,
    Parent,
    Child(String),
}

#[derive(Debug, Eq, PartialEq)]
enum DirChild {
    Dir(String),
    File(u64, String),
}

fn parse_input(input: &'static str) -> Vec<ShellCommand> {
    let mut shell_commands = Vec::new();
    let mut ls_results = None;
    for shell_line in input.lines() {
        let shell_line_tokens = shell_line.split_whitespace().collect::<Vec<&str>>();
        if shell_line_tokens[0] == "$" {
            if let Some(dir_children) = ls_results {
                // Previous list of directory children after ls command ended and can be collected
                shell_commands.push(ShellCommand::Ls(dir_children));
                ls_results = None;
            }

            // cd command can be added without data from subsequent lines
            if shell_line_tokens[1] == "cd" {
                let cd_direction = match shell_line_tokens[2] {
                    "/" => CdDirection::Root,
                    ".." => CdDirection::Parent,
                    c => CdDirection::Child(String::from(c)),
                };
                shell_commands.push(ShellCommand::Cd(cd_direction));
            }
        } else {
            // Lines without a leading $ are directory elements after an ls command
            let current_dir_child_tokens = shell_line.split_whitespace().collect::<Vec<&str>>();
            let current_dir_child = if current_dir_child_tokens[0] == "dir" {
                DirChild::Dir(String::from(current_dir_child_tokens[1]))
            } else {
                DirChild::File(
                    current_dir_child_tokens[0].parse().unwrap(),
                    String::from(current_dir_child_tokens[1]),
                )
            };
            ls_results = if let Some(mut dir_children) = ls_results {
                dir_children.push(current_dir_child);
                Some(dir_children)
            } else {
                Some(vec![current_dir_child])
            };
        }
    }

    if let Some(dir_children) = ls_results {
        // Previous list of directory children after ls command ended and can be collected
        shell_commands.push(ShellCommand::Ls(dir_children));
    }

    shell_commands
}

fn get_all_partial_paths_from_path_elements(path_elements: Vec<&str>) -> Vec<String> {
    let mut path_directories = vec![String::from("/")];

    if !path_elements.is_empty() {
        for i in 0..path_elements.len() {
            let mut path = String::from('/');
            path.push_str(path_elements[0..=i].join("/").as_str());
            path_directories.push(path);
        }
    }

    path_directories
}

fn get_size_sum_of_directory_children(dir_children: &Vec<DirChild>) -> u64 {
    dir_children
        .iter()
        .map(|dir_child| match dir_child {
            DirChild::File(size, _) => *size,
            _ => 0,
        })
        .sum()
}

fn aggregate_directory_sizes_from_shell_lines(
    shell_lines: &Vec<ShellCommand>,
) -> HashMap<String, u64> {
    let mut directory_sizes = HashMap::new();
    let mut current_path = Vec::new();
    for shell_line in shell_lines {
        match shell_line {
            ShellCommand::Cd(direction) => {
                match direction {
                    CdDirection::Root => {
                        current_path = Vec::new();
                    }
                    CdDirection::Parent => {
                        current_path.remove(current_path.len() - 1);
                    }
                    CdDirection::Child(child_dir) => {
                        current_path.push(child_dir.as_str());
                    }
                };
            }
            ShellCommand::Ls(dir_children) => {
                let child_files_size_sum = get_size_sum_of_directory_children(dir_children);

                let paths = get_all_partial_paths_from_path_elements(current_path.clone());

                for path in paths {
                    if let Some(dir_size) = directory_sizes.get(&path) {
                        directory_sizes.insert(path, dir_size + child_files_size_sum);
                    } else {
                        directory_sizes.insert(path, child_files_size_sum);
                    }
                }
            }
        };
    }

    directory_sizes
}

fn main() {
    let input = include_str!("../inputs/data_day_7.txt");
    let shell_commands = parse_input(input);
    let directories = aggregate_directory_sizes_from_shell_lines(&shell_commands);

    // Solution for puzzle 1
    let size_sum_of_directories_above_size_threshold: u64 = directories
        .iter()
        .map(|(_, size)| *size)
        .filter(|size| *size <= 100000)
        .sum();
    println!(
        "Sum of all directory sizes below 100000 is {}",
        size_sum_of_directories_above_size_threshold
    );

    // Solution for puzzle 2
    let used_space = directories.get("/").unwrap();
    let unused_space = 70000000 - used_space;
    let missing_space = 30000000 - unused_space;
    let size_of_smallest_directory_to_gain_missing_space_via_deletion: u64 = directories
        .iter()
        .map(|(_, size)| *size)
        .filter(|size| *size >= missing_space)
        .min()
        .unwrap();
    println!(
        "Missing space of {} can be gained by deleting {}",
        missing_space, size_of_smallest_directory_to_gain_missing_space_via_deletion
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_parsing() {
        let input = "$ cd /
        $ cd ..
        $ cd a
        $ ls
        dir abc
        123 test.txt
        $ cd /
        $ ls
        dir def";

        let shell_command = parse_input(input);
        assert_eq!(
            shell_command,
            vec![
                ShellCommand::Cd(CdDirection::Root),
                ShellCommand::Cd(CdDirection::Parent),
                ShellCommand::Cd(CdDirection::Child(String::from("a"))),
                ShellCommand::Ls(vec![
                    DirChild::Dir(String::from("abc")),
                    DirChild::File(123, String::from("test.txt")),
                ]),
                ShellCommand::Cd(CdDirection::Root),
                ShellCommand::Ls(vec![DirChild::Dir(String::from("def"))]),
            ]
        )
    }

    #[test]
    fn test_partial_paths_from_root_path() {
        let paths = get_all_partial_paths_from_path_elements(Vec::new());
        assert_eq!(paths, vec!["/"]);
    }

    #[test]
    fn test_partial_paths_from_path() {
        let paths = get_all_partial_paths_from_path_elements(vec!["a", "b", "c"]);
        assert_eq!(paths, vec!["/", "/a", "/a/b", "/a/b/c"]);
    }

    #[test]
    fn test_size_sum_of_directory_children() {
        let sizes_sum = get_size_sum_of_directory_children(&vec![
            DirChild::File(1, String::from("a")),
            DirChild::File(2, String::from("b")),
            DirChild::Dir(String::from("c")),
            DirChild::File(3, String::from("d")),
        ]);
        assert_eq!(sizes_sum, 6);
    }

    #[test]
    fn test_size_sum_of_directory_children_without_files() {
        let sizes_sum = get_size_sum_of_directory_children(&vec![DirChild::Dir(String::from("a"))]);
        assert_eq!(sizes_sum, 0);
    }

    #[test]
    fn test_directory_sizes_aggregation() {
        let shell_commands = vec![
            ShellCommand::Cd(CdDirection::Root),
            ShellCommand::Cd(CdDirection::Child(String::from("a"))),
            ShellCommand::Ls(vec![
                DirChild::Dir(String::from("abc")),
                DirChild::File(123, String::from("test.txt")),
            ]),
            ShellCommand::Cd(CdDirection::Root),
            ShellCommand::Ls(vec![DirChild::Dir(String::from("def"))]),
        ];
        let aggregated_directory_sizes =
            aggregate_directory_sizes_from_shell_lines(&shell_commands);

        let correct_sizes = vec![(String::from("/"), 123), (String::from("/a"), 123)]
            .iter()
            .cloned()
            .collect();
        assert_eq!(aggregated_directory_sizes, correct_sizes);
    }
}
