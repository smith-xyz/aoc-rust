use std::env;

use aoc_rust::{cli::runner::Runner, years::run_day};

fn main() {
    let mut args = env::args().skip(1);
    let runner = match Runner::with_args(args.next(), args.next()) {
        Ok(runner) => {
            // If args were provided, run automatically
            if let (Some(year), Some(day)) = (runner.current_year, runner.current_day) {
                run_day(year, day);
                println!();
            }
            runner
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    runner.read_user_input_and_execute()
}
