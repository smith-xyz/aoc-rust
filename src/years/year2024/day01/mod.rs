use crate::{solver::solver::Solver, utils::file_reader::FileReader};

pub struct Day01 {
    data: String,
}

impl Solver<u32> for Day01 {
    fn new<R: FileReader>(reader: &R, file_path: &str) -> Result<Self, String> {
        let data = reader.read_file(file_path)?;
        Ok(Day01 { data })
    }

    fn part_one_solution(&mut self) -> u32 {
        let is_empty = self.data.is_empty();
        if is_empty { 0 } else { 1 }
    }

    fn part_two_solution(&mut self) -> u32 {
        0
    }
}
