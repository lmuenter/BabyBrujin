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


#[test]
fn test_shuffle_stream_input() {
   let input_data = "This is a test data string used for testing input streaming.";
   let mut cmd = Command::cargo_bin("baby_brujin").unwrap();

   let assert = cmd.arg("shuffle")
      .write_stdin(input_data)
      .assert()
      .success();

   let output_str = String::from_utf8(assert.get_output().stdout.clone()).unwrap();

   let not_same_as_input = predicate::function(|x: &str| x != input_data);
   
   assert!(not_same_as_input.eval(&output_str), "Output should be shuffled and different from input");
}
