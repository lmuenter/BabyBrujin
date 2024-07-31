use std::fs;
use rand::{thread_rng, seq::SliceRandom};


pub fn shuffle_text(file_path: &str) -> Result<String, std::io::Error> {
    let content = fs::read_to_string(file_path)?;
    let mut fragments = content.split_whitespace().collect::<Vec<&str>>();
    let mut rng = thread_rng();
    fragments.shuffle(&mut rng);
    Ok(fragments.join(" "))
}
