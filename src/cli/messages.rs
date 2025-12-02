pub struct Messages;

impl Messages {
    pub const USER_PROMPT: &'static str = "Enter year and day to execute solution";
    pub const INVALID_YEAR: &'static str = "Invalid year";
    pub const INVALID_DAY: &'static str = "Invalid day";
    pub const READ_ERROR: &'static str = "Failed to read input";
    pub const NO_SELECTION: &'static str = "No year/day set. Please enter year and day first.";
    pub const EXPECTED_FORMAT: &'static str = "Expected 'year day' format";
    pub const BOTH_REQUIRED: &'static str = "Both year and day must be provided";
    pub const TRY_AGAIN: &'static str = "Error, try again";
    pub const REDO_MESSAGE: &'static str = "(press r to run current selection)";
}
