pub struct StdFileReader;

pub trait FileReader {
    fn read_file(&self, path: &str) -> Result<String, String>;
}

impl FileReader for StdFileReader {
    fn read_file(&self, path: &str) -> Result<String, String> {
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read {}: {}", path, e))
    }
}
