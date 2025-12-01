pub mod year2024;
pub mod year2025; // When you add it

pub fn run_day(year: u32, day: u32) {
    match year {
        2024 => year2024::run_day(day),
        2025 => year2025::run_day(day),
        _ => println!("Year {} not implemented", year),
    }
}
