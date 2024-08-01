use rand::{thread_rng, Rng, seq::SliceRandom};


pub fn slice_text(content: &str, min_length: usize, max_length: usize) -> Result<Vec<String>, std::io::Error> {
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



pub fn shuffle_text(content: &str) -> Result<String, std::io::Error> {
    let mut fragments = content.split_whitespace().collect::<Vec<&str>>();
    let mut rng = thread_rng();
    fragments.shuffle(&mut rng);
    Ok(fragments.join(" "))
}
