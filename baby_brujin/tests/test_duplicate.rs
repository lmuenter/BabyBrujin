use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;


#[test]
fn test_duplicate_file() {
    let file_path = "tests/test_data/test_input.txt";
    let duplication_length = 5;
    let replication_depth = 3;
    let mut cmd = Command::cargo_bin("baby_brujin").unwrap();
    let original_content = fs::read_to_string(file_path)
        .expect("Failed to read test input file");

    cmd.arg("duplicate")
       .arg("--file")
       .arg(file_path)
       .arg("--length-section-duplicated")
       .arg(duplication_length.to_string())
       .arg("--replication-depth")
       .arg(replication_depth.to_string());

    let assert = cmd.assert().success();
    let output = assert.get_output();
    let output_str = String::from_utf8_lossy(&output.stdout).to_string();

    // verify the correct duplication behaviour
    let words: Vec<&str> = original_content.split_whitespace().collect();
    let expected_section = &words[..duplication_length].join(" ");
    let duplication_pattern = expected_section.repeat(replication_depth);
    let contains_pattern = predicate::str::contains(&duplication_pattern);

    assert!(
        contains_pattern.eval(&output_str),
        "Output should contain the correct duplication pattern"
    );

    // ensure output differs from input
    assert_ne!(output_str, original_content, "Output should be different from input when duplications occur");
}


#[test]
fn test_duplicate_stream_input() {
    let input_data = "This is a test data string used for testing input streaming.";
    let duplication_length = 4;
    let replication_depth = 2;
    let mut cmd = Command::cargo_bin("baby_brujin").unwrap();

    let output = cmd.arg("duplicate")
        .write_stdin(input_data)
        .arg("--length-section-duplicated")
        .arg(duplication_length.to_string())
        .arg("--replication-depth")
        .arg(replication_depth.to_string())
        .assert()
        .success()
        .get_output()
        .clone();

    let output_str = String::from_utf8_lossy(&output.stdout).to_string();

    // check for duplication
    let expected_section = input_data.split_whitespace().take(duplication_length).collect::<Vec<_>>().join(" ");
    let duplication_pattern = expected_section.repeat(replication_depth);
    assert!(
        output_str.contains(&duplication_pattern),
        "Output should contain the duplicated section correctly"
    );

    // output should differ from input
    assert_ne!(
        output_str, input_data,
        "Output should be different from the input due to duplications"
    );
}
