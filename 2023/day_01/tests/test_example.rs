use assert_cmd::Command;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[test]
fn test_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day_01")?;

    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push("example.txt");
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let assert = cmd.write_stdin(contents).assert();
    assert.stdout(
        "The sum of calibration values is 142\nThe sum including written calibration values is 142\n",
    );

    Ok(())
}

#[test]
fn test_example2() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day_01")?;

    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push("example2.txt");
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let assert = cmd.write_stdin(contents).assert();
    assert.stdout(
        "The sum of calibration values is 209\nThe sum including written calibration values is 281\n",
    );

    Ok(())
}
