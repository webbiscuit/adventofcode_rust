use assert_cmd::Command;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn read_file(filename: &str) -> String {
    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push(filename);
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

#[test]
fn test_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day_15")?;

    let input = read_file("example.txt");

    cmd.arg("10");
    cmd.arg("0");
    cmd.arg("20");
    let assert = cmd.write_stdin(input).assert();
    assert.stdout("On row 10, there are 26 positions where a beacon can not be present.\nThe distress frequency is 56000011.\n");

    Ok(())
}
