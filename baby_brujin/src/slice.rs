use std::fs;
use rand::{thread_rng, Rng};


pub fn slice_text(file_path: &str, min_length: usize, max_length: usize) -> Result<Vec<String>, std::io::Error> {
    let content = fs::read_to_string(file_path)?;
    let mut rng = thread_rng();
    let mut fragments = Vec::new();
    let mut position = 0;

    // clean content
    let content = content.lines()
                                .filter(|line| !line.trim().is_empty())
                                .collect::<Vec<_>>()
                                .join(" ");

    while position < content.len() {
        let length = rng.gen_range(min_length..=max_length).min(content.len() - position);
        let end = position + length;
        fragments.push(content[position..end].to_string());
        position = end;
    }

    Ok(fragments)
}