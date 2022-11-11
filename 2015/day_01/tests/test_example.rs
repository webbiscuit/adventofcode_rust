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

    cmd.write_stdin(contents)
        .assert()
        .stdout("Santa is on floor -1\nSanta hits the basement on step 5\n");

    Ok(())
}
