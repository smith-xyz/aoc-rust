use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

pub trait BFSState: Clone + Hash + Eq {
    // some helper methods for traveling the bfs search
    fn neighbors(&self) -> Vec<Self>;
    fn is_goal(&self) -> bool;
    fn should_prune(&self) -> bool {
        false // Default: no pruning
    }
}

pub struct BFS<S: BFSState> {
    queue: VecDeque<(S, usize)>,
    seen: HashSet<S>,
}

impl<S: BFSState> BFS<S> {
    pub fn new(initial: S) -> Self {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();

        seen.insert(initial.clone());
        queue.push_back((initial, 0));

        Self { queue, seen }
    }

    pub fn search(&mut self) -> Option<usize> {
        while let Some((state, depth)) = self.queue.pop_front() {
            if state.is_goal() {
                return Some(depth);
            }

            for neighbor in state.neighbors() {
                if neighbor.should_prune() {
                    continue;
                }

                if self.seen.insert(neighbor.clone()) {
                    self.queue.push_back((neighbor, depth + 1));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Hash, PartialEq, Eq, Debug)]
    struct NumberState {
        value: usize,
        target: usize,
    }

    impl BFSState for NumberState {
        fn neighbors(&self) -> Vec<Self> {
            vec![1, 2, 3]
                .into_iter()
                .map(|add| NumberState {
                    value: self.value + add,
                    target: self.target,
                })
                .collect()
        }

        fn is_goal(&self) -> bool {
            self.value == self.target
        }

        fn should_prune(&self) -> bool {
            self.value > self.target
        }
    }

    #[test]
    fn test_simple_number_path() {
        let initial = NumberState {
            value: 0,
            target: 5,
        };
        let mut bfs = BFS::new(initial);
        let result = bfs.search();
        assert_eq!(result, Some(2));
    }

    #[derive(Clone, Hash, PartialEq, Eq, Debug)]
    struct GridState {
        x: i32,
        y: i32,
        target_x: i32,
        target_y: i32,
    }

    impl BFSState for GridState {
        fn neighbors(&self) -> Vec<Self> {
            vec![(1, 0), (-1, 0), (0, 1), (0, -1)]
                .into_iter()
                .map(|(dx, dy)| GridState {
                    x: self.x + dx,
                    y: self.y + dy,
                    target_x: self.target_x,
                    target_y: self.target_y,
                })
                .collect()
        }

        fn is_goal(&self) -> bool {
            self.x == self.target_x && self.y == self.target_y
        }
    }

    #[test]
    fn test_grid_pathfinding() {
        let initial = GridState {
            x: 0,
            y: 0,
            target_x: 2,
            target_y: 2,
        };
        let mut bfs = BFS::new(initial);
        let result = bfs.search();
        assert_eq!(result, Some(4));
    }

    #[derive(Clone, Hash, PartialEq, Eq, Debug)]
    struct BoundedState {
        value: usize,
        target: usize,
        max_value: usize,
    }

    impl BFSState for BoundedState {
        fn neighbors(&self) -> Vec<Self> {
            vec![1, 2]
                .into_iter()
                .map(|add| BoundedState {
                    value: self.value + add,
                    target: self.target,
                    max_value: self.max_value,
                })
                .collect()
        }

        fn is_goal(&self) -> bool {
            self.value == self.target
        }

        fn should_prune(&self) -> bool {
            self.value > self.max_value
        }
    }

    #[test]
    fn test_pruning_prevents_exploration() {
        let initial = BoundedState {
            value: 0,
            target: 5,
            max_value: 10,
        };
        let mut bfs = BFS::new(initial);
        let result = bfs.search();
        assert_eq!(result, Some(3));

        let initial_pruned = BoundedState {
            value: 0,
            target: 5,
            max_value: 4,
        };
        let mut bfs_pruned = BFS::new(initial_pruned);
        let result_pruned = bfs_pruned.search();
        assert_eq!(result_pruned, None);
    }
}
