use assert_cmd::Command;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[test]
fn test_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day_08")?;

    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push("example.txt");
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let assert = cmd.write_stdin(contents).assert();
    assert.stdout(
        "There are 2 steps to reach ZZZ.\nThere is no way for ghosts to reach all the Zs.\n",
    );

    Ok(())
}

#[test]
fn test_example3() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("day_08")?;

    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push("example3.txt");
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let assert = cmd.write_stdin(contents).assert();
    assert.stdout("There is no way to reach ZZZ.\nGhosts take 6 steps to reach all the Zs.\n");

    Ok(())
}
