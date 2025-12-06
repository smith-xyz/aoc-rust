use std::ops::RangeInclusive;

use crate::{solver::solver::Solver, utils::file_reader::FileReader};

pub struct Day05 {
    ranges: Vec<RangeInclusive<u64>>,
    ingredient_ids: Vec<u64>,
}

impl Solver<u64> for Day05 {
    fn new<R: FileReader>(reader: &R, file_path: &str) -> Result<Self, String> {
        let data = reader.read_file(file_path)?;
        let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();
        let mut ingredient_ids: Vec<u64> = Vec::new();

        data.split("\n").for_each(|s| {
            if s.contains("-") {
                let values: Vec<u64> = s.split("-").map(|x| x.parse::<u64>().unwrap()).collect();
                ranges.push(values[0]..=values[1]);
            }
            if s.parse::<u64>().is_ok() {
                ingredient_ids.push(s.parse::<u64>().unwrap());
            }
        });

        Ok(Day05 {
            ranges,
            ingredient_ids,
        })
    }

    fn part_one_solution(&mut self) -> u64 {
        let mut fresh_ingredient_ids: Vec<u64> = Vec::new();
        for id in &self.ingredient_ids {
            let in_range = self.ranges.iter().any(|range| range.contains(&id));
            if in_range {
                fresh_ingredient_ids.push(*id)
            }
        }
        fresh_ingredient_ids.iter().count() as u64
    }

    fn part_two_solution(&mut self) -> u64 {
        self.ranges.sort_by_key(|r| *r.start());
        let mut merged: Vec<RangeInclusive<u64>> = Vec::new();
        for range in &self.ranges {
            if let Some(last) = merged.last_mut() {
                if *range.start() <= *last.end() + 1 {
                    *last = *last.start()..=*range.end().max(last.end());
                    continue;
                }
            }
            merged.push(*range.start()..=*range.end());
        }
        merged.iter().map(|r| *r.end() - *r.start() + 1).sum()
    }
}
