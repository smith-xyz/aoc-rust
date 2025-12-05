use crate::{solver::solver::Solver, utils::file_reader::FileReader};

pub struct Day03 {
    data: Vec<Vec<u64>>,
}

impl Solver<u64> for Day03 {
    fn new<R: FileReader>(reader: &R, file_path: &str) -> Result<Self, String> {
        let data = reader
            .read_file(file_path)?
            .split("\n")
            .map(|x| x.chars().map(|c| c.to_digit(10).unwrap() as u64).collect())
            .collect();
        Ok(Day03 { data })
    }

    fn part_one_solution(&mut self) -> u64 {
        let mut largest: Vec<u64> = vec![];
        for bank in self.data.clone() {
            largest.push(find_largest_n_digit(&bank, 2));
        }
        largest.iter().sum()
    }

    fn part_two_solution(&mut self) -> u64 {
        let mut largest: Vec<u64> = vec![];
        for bank in self.data.clone() {
            largest.push(find_largest_n_digit(&bank, 12));
        }
        largest.iter().sum()
    }
}

fn find_largest_n_digit(bank: &[u64], n: usize) -> u64 {
    let mut result = bank[0..n].to_vec();
    let mut result_positions = (0..n).collect::<Vec<usize>>();

    for (i, num) in bank.iter().enumerate().skip(1) {
        if *num > result[0] && i + (n - 1) < bank.len() {
            result[0] = *num;
            result_positions[0] = i;
            for pos in 1..n {
                result[pos] = bank[i + pos];
                result_positions[pos] = i + pos;
            }
            continue;
        }

        for pos in (1..n).rev() {
            if i > result_positions[pos - 1] && *num > result[pos] && bank.len() - i >= n - pos {
                result[pos] = *num;
                result_positions[pos] = i;
                // shift everything after
                for fill_pos in (pos + 1)..n {
                    result[fill_pos] = bank[i + (fill_pos - pos)];
                    result_positions[fill_pos] = i + (fill_pos - pos);
                }
                continue;
            }
        }
    }

    let joined: String = result.iter().map(|d| d.to_string()).collect();
    format!("{}", joined).parse::<u64>().unwrap()
}
