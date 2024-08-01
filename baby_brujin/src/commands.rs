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


pub fn duplicate(content: &str, length_section_duplicated: usize, replication_depth: usize) -> Result<String, std::io::Error> {
    let mut rng = thread_rng();
    let fragments = content.split_whitespace();
    let words = fragments.collect::<Vec<&str>>();
    let content_length = words.len();

    // join stretch of the content of length `length_section_duplicated`, but only of adjacent indices
    if content_length < length_section_duplicated || replication_depth == 0 {
        return Ok(content.to_string());
    }

    let start_index = rng.gen_range(0..content_length - length_section_duplicated + 1);
    let end_index = start_index + length_section_duplicated - 1;

    let section_to_duplicate = &words[start_index..end_index];

    // repeat stretch `replication_depth` times
    let mut result = Vec::with_capacity(content_length + length_section_duplicated * replication_depth);
    result.extend_from_slice(&words[0..start_index]);

    for _ in 0..replication_depth {
        result.extend_from_slice(section_to_duplicate);
    }


    // return string with duplicated section
    result.extend_from_slice(&words[end_index..]);
    Ok(result.join(" "))
}