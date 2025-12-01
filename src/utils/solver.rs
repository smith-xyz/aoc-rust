use crate::utils::file_reader::FileReader;

pub trait Solver {
    fn input_path(year: u32, day: u32) -> String {
        format!("src/years/year{}/day{:02}/input.txt", year, day)
    }

    fn from_default_path<R: FileReader>(reader: &R, year: u32, day: u32) -> Result<Self, String>
    where
        Self: Sized,
    {
        Self::new(reader, &Self::input_path(year, day))
    }

    fn new<R: FileReader>(reader: &R, file_path: &str) -> Result<Self, String>
    where
        Self: Sized;

    fn part_one_solution(&self) -> u32;
    fn part_two_solution(&self) -> u32;

    fn solve(&self) {
        println!("Part 1: {}", self.part_one_solution());
        println!("Part 2: {}", self.part_two_solution());
    }
}
