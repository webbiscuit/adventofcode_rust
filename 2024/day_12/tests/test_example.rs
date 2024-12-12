use assert_cmd::Command;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[test]
fn test_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day_12")?;

    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push("example3.txt");
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let assert = cmd.write_stdin(contents).assert();
    assert.stdout("The total price of fence is 1930\nThe total price of fence with discount is 1206\n");

    Ok(())
}
