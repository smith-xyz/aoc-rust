use crate::utils::{file_reader::StdFileReader, solver::Solver};

pub mod day01;

pub fn run_day(day: u32) {
    let reader = StdFileReader;
    match day {
        1 => {
            let solver =
                day01::Day01::from_default_path(&reader, 2024, day).expect("Failed to load input");
            solver.solve();
        }
        _ => println!("Day {} not implemented", day),
    }
}
