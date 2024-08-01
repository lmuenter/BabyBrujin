use std::io::{self, Read};


pub fn read_input(source: Option<String>) -> Result<String, std::io::Error> {
    match source {
        Some(file_path) => std::fs::read_to_string(file_path),
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            Ok(buffer)
        }
    }
}