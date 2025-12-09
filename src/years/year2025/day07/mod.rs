use crate::{solver::solver::Solver, utils::file_reader::FileReader};
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Day07 {
    caret_hits: u128,
    timelines: u128,
}

impl Solver<u128> for Day07 {
    fn new<R: FileReader>(reader: &R, file_path: &str) -> Result<Self, String> {
        let data = reader.read_file(file_path)?;
        let grid: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
        let starting_pos = (
            0 as usize,
            grid[0].iter().position(|&x| x == 'S').unwrap() as usize,
        );
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut caret_hits: HashSet<(usize, usize)> = HashSet::new();
        queue.push_back(starting_pos);
        while let Some((row, col)) = queue.pop_front() {
            if row >= grid.len() || col >= grid[row].len() || caret_hits.contains(&(row, col)) {
                continue;
            }

            match grid[row][col] {
                '^' => {
                    caret_hits.insert((row, col));
                    if col > 0 {
                        queue.push_back((row, col - 1));
                    }
                    if col + 1 < grid[row].len() {
                        queue.push_back((row, col + 1));
                    }
                }
                '.' | 'S' => {
                    if row + 1 < grid.len() {
                        queue.push_back((row + 1, col));
                    }
                }
                _ => continue,
            }
        }

        // because bfs is fighting me so much
        let mut path_counts: HashMap<usize, u128> = HashMap::new();
        path_counts.insert(starting_pos.1, 1);

        for row in &grid {
            let mut new_counts: HashMap<usize, u128> = HashMap::new();

            for (&col, &count) in &path_counts {
                match row[col] {
                    '^' => {
                        if col > 0 {
                            *new_counts.entry(col - 1).or_insert(0) += count;
                        }
                        if col + 1 < row.len() {
                            *new_counts.entry(col + 1).or_insert(0) += count;
                        }
                    }
                    '.' | 'S' => {
                        *new_counts.entry(col).or_insert(0) += count;
                    }
                    _ => {}
                }
            }

            path_counts = new_counts;
        }

        Ok(Day07 {
            caret_hits: caret_hits.len() as u128,
            timelines: path_counts.values().copied().sum::<u128>(),
        })
    }

    fn part_one_solution(&mut self) -> u128 {
        self.caret_hits
    }

    fn part_two_solution(&mut self) -> u128 {
        self.timelines
    }
}
