use crate::{solver::solver::Solver, utils::file_reader::StdFileReader};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;

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
        5 => {
            let mut solver =
                day05::Day05::from_default_path(&reader, 2025, day).expect("Failed to load input");
            solver.solve();
        }
        6 => {
            let mut solver =
                day06::Day06::from_default_path(&reader, 2025, day).expect("Failed to load input");
            solver.solve();
        }
        7 => {
            let mut solver =
                day07::Day07::from_default_path(&reader, 2025, day).expect("Failed to load input");
            solver.solve();
        }
        8 => {
            let mut solver =
                day08::Day08::from_default_path(&reader, 2025, day).expect("Failed to load input");
            solver.solve();
        }
        9 => {
            let mut solver =
                day09::Day09::from_default_path(&reader, 2025, day).expect("Failed to load input");
            solver.solve();
        }
        10 => {
            let mut solver =
                day10::Day10::from_default_path(&reader, 2025, day).expect("Failed to load input");
            solver.solve();
        }
        _ => println!("Day {} not implemented", day),
    }
}
