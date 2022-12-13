use std::collections::{HashMap, HashSet};

struct HillClimbingGraph {
    start_nodes: HashSet<(usize, usize)>,
    goal_node: (usize, usize),
    adjacency_list: HashMap<(usize, usize), HashSet<(usize, usize)>>,
}

impl HillClimbingGraph {
    fn new(input: &Vec<Vec<char>>, use_single_start_node: bool) -> Self {
        let mut start_nodes = HashSet::new();
        let mut goal_node = None;
        let mut adjacency_list = HashMap::new();

        for (y, line) in input.iter().enumerate() {
            for (x, hill) in line.iter().enumerate() {
                if *hill == 'S' || (!use_single_start_node && *hill == 'a') {
                    start_nodes.insert((x, y));
                } else if *hill == 'E' {
                    goal_node = Some((x, y));
                }
                let height = get_hill_height(*hill);
                let mut hill_adjacency = HashSet::new();
                for (n_x, n_y, n_height) in get_neighbour_heights(x, y, input) {
                    if n_height <= (height + 1) {
                        hill_adjacency.insert((n_x, n_y));
                    }
                }

                adjacency_list.insert((x, y), hill_adjacency);
            }
        }

        Self {
            start_nodes,
            goal_node: goal_node.unwrap(),
            adjacency_list,
        }
    }
}

fn get_hill_height(c: char) -> u32 {
    match c {
        'S' => 1,
        'E' => 26,
        _ => u32::from(c) - 96,
    }
}

fn get_neighbour_heights(x: usize, y: usize, input: &Vec<Vec<char>>) -> Vec<(usize, usize, u32)> {
    let mut neighbors = Vec::new();

    if x > 0 {
        neighbors.push((x - 1, y, get_hill_height(input[y][x - 1])));
    }
    if x < input[y].len() - 1 {
        neighbors.push((x + 1, y, get_hill_height(input[y][x + 1])));
    }
    if y > 0 {
        neighbors.push((x, y - 1, get_hill_height(input[y - 1][x])));
    }
    if y < input.len() - 1 {
        neighbors.push((x, y + 1, get_hill_height(input[y + 1][x])));
    }

    neighbors
}

fn parse_input(input: &'static str, use_single_start_node: bool) -> HillClimbingGraph {
    let hill_map = input.lines().map(|l| l.trim().chars().collect()).collect();

    HillClimbingGraph::new(&hill_map, use_single_start_node)
}

fn shortest_path_length_search(graph: &HillClimbingGraph) -> Option<usize> {
    let mut search_queue = Vec::new();
    let mut open = HashSet::new();
    let mut costs = HashMap::new();
    let mut closed = HashSet::new();

    for s in &graph.start_nodes {
        search_queue.push((s, 0));
        open.insert(s);
        costs.insert(s, 0);
    }

    while let Some((coordinates, cost)) = search_queue.pop() {
        open.remove(&coordinates);
        closed.insert(coordinates);

        if let Some(neighbors) = graph.adjacency_list.get(coordinates) {
            for neighbor in neighbors {
                let neighbor_cost = cost + 1;

                if let Some(current_cost) = costs.get(neighbor) {
                    if *current_cost > neighbor_cost {
                        costs.insert(neighbor, neighbor_cost);
                        closed.remove(neighbor);
                    }
                } else {
                    costs.insert(neighbor, neighbor_cost);
                }

                if !open.contains(neighbor) && !closed.contains(neighbor) {
                    open.insert(neighbor);
                    search_queue.push((neighbor, neighbor_cost));
                }
            }
        }
    }

    costs.get(&graph.goal_node).cloned()
}

fn main() {
    let input = include_str!("../inputs/data_day_12.txt");

    // Solution for puzzle 1
    let hill_graph = parse_input(input, true);
    let shortest_path = shortest_path_length_search(&hill_graph).unwrap();
    println!(
        "Shortest path from start to the hill with the best signal has length {}",
        shortest_path
    );

    // Solution for puzzle 2
    let hill_graph = parse_input(input, false);
    let shortest_path = shortest_path_length_search(&hill_graph).unwrap();
    println!(
        "Shortest path from any low-level hill to the hill with the best signal has length {}",
        shortest_path
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hill_height() {
        assert_eq!(get_hill_height('S'), 1);
        assert_eq!(get_hill_height('E'), 26);
        assert_eq!(get_hill_height('a'), 1);
        assert_eq!(get_hill_height('z'), 26);
    }

    #[test]
    fn test_get_neighbor_heights() {
        let heights = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ];
        assert_eq!(
            get_neighbour_heights(0, 0, &heights),
            vec![(1, 0, 2), (0, 1, 4)]
        );
        assert_eq!(
            get_neighbour_heights(2, 0, &heights),
            vec![(1, 0, 2), (2, 1, 6)]
        );
        assert_eq!(
            get_neighbour_heights(0, 2, &heights),
            vec![(1, 2, 8), (0, 1, 4)]
        );
        assert_eq!(
            get_neighbour_heights(2, 2, &heights),
            vec![(1, 2, 8), (2, 1, 6)]
        );
        assert_eq!(
            get_neighbour_heights(1, 1, &heights),
            vec![(0, 1, 4), (2, 1, 6), (1, 0, 2), (1, 2, 8)]
        );
    }

    #[test]
    fn test_input_parsing() {
        let input = "Sbc\nfed\nghE";
        let graph = parse_input(input, true);

        assert_eq!(graph.start_nodes, HashSet::from([(0, 0)]));
        assert_eq!(graph.goal_node, (2, 2));
        assert_eq!(graph.adjacency_list.len(), 9);
        assert_eq!(
            graph.adjacency_list.get(&(0, 0)).unwrap().clone(),
            HashSet::from([(1, 0)])
        );
        assert_eq!(
            graph.adjacency_list.get(&(1, 0)).unwrap().clone(),
            HashSet::from([(0, 0), (2, 0)])
        );
        assert_eq!(
            graph.adjacency_list.get(&(2, 0)).unwrap().clone(),
            HashSet::from([(1, 0), (2, 1)])
        );
        assert_eq!(
            graph.adjacency_list.get(&(2, 1)).unwrap().clone(),
            HashSet::from([(2, 0), (1, 1)])
        );
        assert_eq!(
            graph.adjacency_list.get(&(1, 1)).unwrap().clone(),
            HashSet::from([(2, 1), (1, 0), (0, 1)])
        );
        assert_eq!(
            graph.adjacency_list.get(&(0, 1)).unwrap().clone(),
            HashSet::from([(1, 1), (0, 0), (0, 2)])
        );
        assert_eq!(
            graph.adjacency_list.get(&(0, 2)).unwrap().clone(),
            HashSet::from([(0, 1), (1, 2)])
        );
        assert_eq!(
            graph.adjacency_list.get(&(1, 2)).unwrap().clone(),
            HashSet::from([(0, 2), (1, 1)])
        );
        assert_eq!(
            graph.adjacency_list.get(&(2, 2)).unwrap().clone(),
            HashSet::from([(1, 2), (2, 1)])
        );
    }

    #[test]
    fn test_path_search() {
        let graph = HillClimbingGraph {
            start_nodes: HashSet::from([(0, 0)]),
            goal_node: (1, 1),
            adjacency_list: HashMap::from([
                ((0, 0), HashSet::from([(1, 0), (0, 1)])),
                ((1, 0), HashSet::new()),
                ((0, 1), HashSet::from([(1, 1)])),
            ]),
        };
        let path = shortest_path_length_search(&graph).unwrap();
        assert_eq!(path, 2);
    }
}
