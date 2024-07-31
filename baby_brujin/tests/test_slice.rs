use assert_cmd::Command;

#[test]
fn test_slice_file() {
    let file_path = "tests/test_data/test_input.txt";
    let min_length = 9;
    let max_length = 17;
    let mut cmd = Command::cargo_bin("baby_brujin").unwrap();

    // setup of command
    cmd.arg("slice")
       .arg("--file")
       .arg(file_path)
       .arg("--min-length")
       .arg(min_length.to_string())
       .arg("--max-length")
       .arg(max_length.to_string());

    // execute and capture
    let assert = cmd.assert().success();
    let output = assert.get_output();
    let output_str = String::from_utf8_lossy(&output.stdout);

    // split
    let fragments: Vec<&str> = output_str
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    let last_fragment = fragments.last().unwrap();

    // check fragment lengths
    for &fragment in &fragments[..fragments.len() - 1] {
        assert!(fragment.len() >= min_length && fragment.len() <= max_length,
                "Fragment '{}' length {} should be within the range {} to {}",
                fragment, fragment.len(), min_length, max_length);
    }

    assert!(last_fragment.len() <= max_length,
            "Last fragment '{}' length {} should not exceed {}",
            last_fragment, last_fragment.len(), max_length);
}
