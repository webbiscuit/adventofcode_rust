use assert_cmd::Command;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[test]
fn test_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day_14")?;
    cmd.arg("11");
    cmd.arg("7");

    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push("example.txt");
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let assert = cmd.write_stdin(contents).assert();
    assert.stdout("Safety factor is 12\n");

    Ok(())
}
