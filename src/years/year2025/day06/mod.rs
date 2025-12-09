use std::collections::HashMap;

use crate::{solver::solver::Solver, utils::file_reader::FileReader};

pub struct Day06 {
    operation_order: Vec<char>,
    num_sets: HashMap<usize, Vec<Vec<Option<char>>>>,
}

impl Solver<u128> for Day06 {
    fn new<R: FileReader>(reader: &R, file_path: &str) -> Result<Self, String> {
        let data = reader.read_file(file_path)?;
        let mut lines: Vec<&str> = data.split("\n").collect();
        let operation_order: Vec<char> = lines
            .pop()
            .unwrap()
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.trim().chars().next().unwrap())
            .collect();
        let mut formatted: Vec<Vec<String>> = Vec::new();
        for line in &lines {
            formatted.push(
                line.split("")
                    .filter(|x| x.len() > 0)
                    .map(|x| {
                        if x == " " {
                            "NONE".to_string()
                        } else {
                            x.to_string()
                        }
                    })
                    .collect::<Vec<String>>(),
            );
        }

        let num_positions = formatted[0].len();
        let mut is_separator: Vec<bool> = vec![true; num_positions];
        for row in &formatted {
            for (i, ch) in row.iter().enumerate() {
                if ch != "NONE" {
                    is_separator[i] = false;
                }
            }
        }

        let mut num_sets: HashMap<usize, Vec<Vec<Option<char>>>> = HashMap::new();
        let mut col_idx = 0;
        let mut field_start = 0;
        let mut in_field = false;

        for i in 0..=num_positions {
            let is_sep = if i < num_positions {
                is_separator[i]
            } else {
                true
            };

            if !is_sep && !in_field {
                field_start = i;
                in_field = true;
            } else if is_sep && in_field {
                for row in &formatted {
                    let field_chars: Vec<Option<char>> = row[field_start..i]
                        .iter()
                        .map(|s| if s == "NONE" { None } else { s.chars().next() })
                        .collect();
                    num_sets
                        .entry(col_idx)
                        .or_insert_with(Vec::new)
                        .push(field_chars);
                }
                col_idx += 1;
                in_field = false;
            }
        }
        Ok(Day06 {
            operation_order,
            num_sets,
        })
    }

    fn part_one_solution(&mut self) -> u128 {
        let mut total: u128 = 0;
        for (i, symbol) in self.operation_order.iter().enumerate() {
            let parsed_nums = self.num_sets[&i].iter().map(|x| {
                x.iter()
                    .filter_map(|opt| *opt)
                    .collect::<String>()
                    .parse::<u128>()
                    .unwrap()
            });
            match symbol {
                '*' => total += parsed_nums.product::<u128>(),
                '+' => total += parsed_nums.sum::<u128>(),
                _ => panic!("unhandled symbol: {}", symbol),
            }
        }
        total
    }

    fn part_two_solution(&mut self) -> u128 {
        let mut total: u128 = 0;

        for i in (0..self.operation_order.len()).rev() {
            let symbol = self.operation_order[i];
            let rows = &self.num_sets[&i];
            let max_len = rows.iter().map(|row| row.len()).max().unwrap_or(0);

            let transposed: Vec<Vec<Option<char>>> = (0..max_len)
                .map(|digit_pos| {
                    rows.iter()
                        .map(|row| row.get(digit_pos).copied().flatten())
                        .collect()
                })
                .collect();

            let parsed_nums: Vec<u128> = transposed
                .iter()
                .rev()
                .map(|digit_col| {
                    digit_col
                        .iter()
                        .filter_map(|opt| *opt)
                        .collect::<String>()
                        .parse::<u128>()
                        .unwrap_or(0)
                })
                .filter(|&n| n > 0)
                .collect();

            match symbol {
                '*' => total += parsed_nums.iter().product::<u128>(),
                '+' => total += parsed_nums.iter().sum::<u128>(),
                _ => panic!("unhandled symbol: {}", symbol),
            }
        }
        total
    }
}
