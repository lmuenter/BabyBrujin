use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;


#[test]
fn test_shuffle_file() {
    let file_path = "tests/test_data/test_input.txt";
    let mut cmd = Command::cargo_bin("baby_brujin").unwrap();
    let original_content = fs::read_to_string(file_path)
    .expect("Failed to read test input file");

    cmd.arg("shuffle")
       .arg("--file")
       .arg(file_path);


    let not_same_as_input = predicate::function(|x: &str| x != original_content);


    cmd.assert()
       .success()
       .stdout(predicate::str::contains("Lorem")
               .and(predicate::str::contains("ipsum"))
               .and(predicate::str::contains("dolor"))
               .and(not_same_as_input));
}
