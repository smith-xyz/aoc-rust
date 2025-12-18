use std::collections::HashMap;

use crate::{solver::solver::Solver, utils::file_reader::FileReader};

pub struct Day11 {
    data: Vec<Node>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Node {
    name: String,
    children: Vec<String>,
    parent: Option<Box<Node>>,
}

impl Solver<i64> for Day11 {
    fn new<R: FileReader>(reader: &R, file_path: &str) -> Result<Self, String> {
        let data = reader.read_file(file_path)?;
        let nodes: Vec<Node> = data
            .lines()
            .map(|line| {
                let (name, children) = line.split_once(": ").unwrap();
                let children = children.split(" ").map(|c| c.to_string()).collect();
                Node {
                    name: name.to_string(),
                    children,
                    parent: None, // will be set later
                }
            })
            .collect();

        let mut nodes_clone = nodes.clone();
        for node in nodes_clone.iter_mut() {
            let parent: Option<Box<Node>> = nodes
                .iter()
                .find(|n| n.children.contains(&node.name))
                .map(|n| Box::new(n.clone()));
            node.parent = parent.clone();
        }

        Ok(Day11 { data: nodes_clone })
    }

    fn part_one_solution(&mut self) -> i64 {
        let root = self.data.iter().find(|n| n.name == "you").unwrap();
        count_paths_dfs(&self.data, root, "out")
    }

    fn part_two_solution(&mut self) -> i64 {
        let root = self.data.iter().find(|n| n.name == "svr").unwrap();
        let mut memo = HashMap::new();
        count_paths_dfs_with_memo(&self.data, root, false, false, &mut memo)
    }
}

fn count_paths_dfs(data: &Vec<Node>, current: &Node, target: &str) -> i64 {
    if current.children.contains(&target.to_string()) {
        return 1;
    }

    let mut count = 0;

    for child_name in &current.children {
        if let Some(child_node) = data.iter().find(|n| n.name == *child_name) {
            count += count_paths_dfs(&data, child_node, target);
        }
    }

    count
}

type Memo = HashMap<(String, bool, bool), i64>;

fn count_paths_dfs_with_memo(
    data: &Vec<Node>,
    current: &Node,
    seen_fft: bool,
    seen_dac: bool,
    memo: &mut Memo,
) -> i64 {
    let seen_fft = seen_fft || current.name == "fft";
    let seen_dac = seen_dac || current.name == "dac";

    let key = (current.name.clone(), seen_fft, seen_dac);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    if current.children.contains(&"out".to_string()) {
        let result = if seen_fft && seen_dac { 1 } else { 0 };
        memo.insert(key, result);
        return result;
    }

    let mut total = 0;
    for child_name in &current.children {
        if let Some(child_node) = data.iter().find(|n| n.name == *child_name) {
            total += count_paths_dfs_with_memo(data, child_node, seen_fft, seen_dac, memo);
        }
    }

    memo.insert(key, total);
    total
}

#[cfg(test)]
mod tests {
    use crate::utils::file_reader::StdFileReader;

    use super::*;

    #[test]
    fn test_part_one() {
        let reader = StdFileReader;
        let mut solver = Day11::from_test_path(&reader, 2025, 11).expect("Failed to load input");
        assert_eq!(solver.part_one_solution(), 5)
    }

    #[test]
    fn test_part_two() {
        let reader = StdFileReader;
        let mut solver =
            Day11::from_test_path_part_two(&reader, 2025, 11).expect("Failed to load input");
        assert_eq!(solver.part_two_solution(), 2)
    }
}
