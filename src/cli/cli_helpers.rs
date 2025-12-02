use crate::cli::messages::Messages;

pub fn parse_args(year_str: &str, day_str: &str) -> Result<(u32, u32), String> {
    let year = year_str
        .parse()
        .map_err(|_| Messages::INVALID_YEAR.to_string())?;
    let day = day_str
        .parse()
        .map_err(|_| Messages::INVALID_DAY.to_string())?;
    Ok((year, day))
}

pub fn read_input() -> Result<String, String> {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|_| Messages::READ_ERROR.to_string())?;
    Ok(input.trim().to_string())
}
