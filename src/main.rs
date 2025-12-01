use aoc_rust::years::run_day;

struct Runner {}

impl Runner {
    fn read_user_input_and_execute() {
        loop {
            println!("Enter year and day to execute solution:");
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read");
            let input = input.trim();
            if input == "quite" {
                break;
            } else {
                match Runner::parse_user_input(input) {
                    Ok((year, day)) => run_day(year, day),
                    Err(msg) => {
                        println!("Error, try again: {}", msg);
                        continue;
                    }
                }
            }
        }
    }

    fn parse_user_input(input: &str) -> Result<(u32, u32), String> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        match parts.as_slice() {
            [year_str, day_str] => {
                let year: u32 = year_str.parse().map_err(|_| "Invalid year".to_string())?;
                let day: u32 = day_str.parse().map_err(|_| "Invalid day".to_string())?;
                Ok((year, day))
            }
            _ => Err("Expected 'year day' format".to_string()),
        }
    }
}

fn main() {
    Runner::read_user_input_and_execute()
}
