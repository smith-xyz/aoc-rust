use crate::{
    cli::{
        cli_helpers::{parse_args, read_input},
        messages::Messages,
    },
    years::run_day,
};

pub struct Runner {
    pub current_year: Option<u32>,
    pub current_day: Option<u32>,
}

impl Default for Runner {
    fn default() -> Self {
        Self {
            current_year: None,
            current_day: None,
        }
    }
}

impl Runner {
    pub fn with_args(year_str: Option<String>, day_str: Option<String>) -> Result<Self, String> {
        match (year_str, day_str) {
            (Some(y), Some(d)) => {
                let (year, day) = parse_args(&y, &d)?;
                Ok(Runner {
                    current_year: Some(year),
                    current_day: Some(day),
                })
            }
            (None, None) => Ok(Runner::default()),
            _ => Err(Messages::BOTH_REQUIRED.to_string()),
        }
    }

    pub fn read_user_input_and_execute(mut self) {
        loop {
            println!("{}", self.prompt());
            let input = match read_input() {
                Ok(input) => input,
                Err(e) => {
                    println!("{}: {}", Messages::TRY_AGAIN, e);
                    continue;
                }
            };
            if input == "quit" {
                break;
            }

            match self.parse_user_input(&input) {
                Ok((year, day)) => {
                    self.current_year = Some(year);
                    self.current_day = Some(day);
                    run_day(year, day)
                }
                Err(msg) => {
                    self.current_year = None;
                    self.current_day = None;
                    println!("{}: {}", Messages::TRY_AGAIN, msg);
                    continue;
                }
            }
        }
    }

    fn has_selection(&self) -> bool {
        matches!((self.current_year, self.current_day), (Some(_), Some(_)))
    }

    fn prompt(&self) -> String {
        if self.has_selection() {
            format!("{} {}:", Messages::USER_PROMPT, Messages::REDO_MESSAGE,)
        } else {
            format!("{}:", Messages::USER_PROMPT)
        }
    }

    fn parse_user_input(&self, input: &str) -> Result<(u32, u32), String> {
        if input == "r" {
            return match (self.current_year, self.current_day) {
                (Some(year), Some(day)) => Ok((year, day)),
                _ => Err(Messages::NO_SELECTION.to_string()),
            };
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        match parts.as_slice() {
            [year_str, day_str] => {
                let year: u32 = year_str
                    .parse()
                    .map_err(|_| Messages::INVALID_YEAR.to_string())?;
                let day: u32 = day_str
                    .parse()
                    .map_err(|_| Messages::INVALID_DAY.to_string())?;
                Ok((year, day))
            }
            _ => Err(Messages::EXPECTED_FORMAT.to_string()),
        }
    }
}
