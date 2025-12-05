use crate::{solver::solver::Solver, utils::file_reader::StdFileReader};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

pub fn run_day(day: u32) {
    let reader = StdFileReader;
    match day {
        1 => {
            let mut solver =
                day01::Day01::from_default_path(&reader, 2025, day).expect("Failed to load input");
            solver.solve();
        }
        2 => {
            let mut solver =
                day02::Day02::from_default_path(&reader, 2025, day).expect("Failed to load input");
            solver.solve();
        }
        3 => {
            let mut solver =
                day03::Day03::from_default_path(&reader, 2025, day).expect("Failed to load input");
            solver.solve();
        }
        4 => {
            let mut solver =
                day04::Day04::from_default_path(&reader, 2025, day).expect("Failed to load input");
            solver.solve();
        }
        _ => println!("Day {} not implemented", day),
    }
}
