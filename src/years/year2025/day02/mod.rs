use crate::{
    solver::solver::Solver,
    utils::{file_reader::FileReader, math_utils::find_divisors},
};

pub struct Day02 {
    id_ranges: Vec<(u64, u64)>,
}

impl Solver<u64> for Day02 {
    fn new<R: FileReader>(reader: &R, file_path: &str) -> Result<Self, String> {
        let data = reader.read_file(file_path)?;
        let parsed = data
            .split(',')
            .map(|s| {
                let mut parts = s.split('-');
                (
                    parts.next().unwrap().parse::<u64>().unwrap(),
                    parts.next().unwrap().parse::<u64>().unwrap(),
                )
            })
            .collect();
        Ok(Day02 { id_ranges: parsed })
    }

    fn part_one_solution(&mut self) -> u64 {
        self.id_ranges
            .iter()
            .flat_map(|(start, end)| *start..=*end)
            .filter(|&n| is_invalid_part_one(n))
            .sum()
    }

    fn part_two_solution(&mut self) -> u64 {
        self.id_ranges
            .iter()
            .flat_map(|(start, end)| *start..=*end)
            .filter(|&n| is_invalid_part_two(n))
            .sum()
    }
}

fn is_invalid_part_one(n: u64) -> bool {
    let num_str = n.to_string();
    num_str.len() % 2 == 0 && {
        let (first, second) = num_str.split_at(num_str.len() / 2);
        first == second
    }
}

fn is_invalid_part_two(n: u64) -> bool {
    let num_str = n.to_string();
    let len = num_str.len();

    find_divisors(&num_str)
        .into_iter()
        .filter(|&d| len / d >= 2)
        .any(|d| {
            let first_segment = &num_str[0..d];
            (d..len)
                .step_by(d)
                .all(|i| &num_str[i..i + d] == first_segment)
        })
}
