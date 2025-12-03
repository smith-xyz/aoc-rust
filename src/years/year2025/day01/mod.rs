use crate::{
    solver::solver::Solver, toolbox::combination_safe::CombinationSafe,
    utils::file_reader::FileReader,
};

pub struct Day01 {
    data: String,
}

impl Solver<u32> for Day01 {
    fn new<R: FileReader>(reader: &R, file_path: &str) -> Result<Self, String> {
        let data = reader.read_file(file_path)?;
        Ok(Day01 { data })
    }

    fn part_one_solution(&self) -> u32 {
        let mut combination_safe: CombinationSafe = CombinationSafe::default();
        let dials: Vec<&str> = self.data.split("\n").collect();
        combination_safe.process_dials(&dials).0 as u32
    }

    fn part_two_solution(&self) -> u32 {
        let mut combination_safe: CombinationSafe = CombinationSafe::default();
        let dials: Vec<&str> = self.data.split("\n").collect();
        (|(a, b)| (a + b) as u32)(combination_safe.process_dials(&dials))
    }
}
