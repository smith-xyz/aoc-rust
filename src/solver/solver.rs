use crate::utils::file_reader::FileReader;

pub trait Solver<Output>
where
    Output: std::fmt::Display,
{
    fn input_path(year: u32, day: u32) -> String {
        format!("src/years/year{}/day{:02}/input.txt", year, day)
    }

    fn test_input_path(year: u32, day: u32) -> String {
        format!("src/years/year{}/day{:02}/input_test.txt", year, day)
    }

    fn from_default_path<R: FileReader>(reader: &R, year: u32, day: u32) -> Result<Self, String>
    where
        Self: Sized,
    {
        Self::new(reader, &Self::input_path(year, day))
    }

    fn from_test_path<R: FileReader>(reader: &R, year: u32, day: u32) -> Result<Self, String>
    where
        Self: Sized,
    {
        Self::new(reader, &Self::test_input_path(year, day))
    }

    fn new<R: FileReader>(reader: &R, file_path: &str) -> Result<Self, String>
    where
        Self: Sized;

    fn part_one_solution(&mut self) -> Output;
    fn part_two_solution(&mut self) -> Output;

    fn solve(&mut self) {
        use std::time::Instant;

        let start = Instant::now();
        let part1 = self.part_one_solution();
        let part1_time = start.elapsed();

        let start = Instant::now();
        let part2 = self.part_two_solution();
        let part2_time = start.elapsed();

        println!("Part 1: {} (took {:?})", part1, part1_time);
        println!("Part 2: {} (took {:?})", part2, part2_time);
    }
}
