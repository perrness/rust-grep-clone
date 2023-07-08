use std::process::Command;

use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};
use assert_fs::prelude::FileWriteStr;
use predicates::prelude::predicate;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rust-grep-clone")?;

    cmd.arg("--pattern")
        .arg("foobar")
        .arg("--filepath")
        .arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("rust-grep-clone")?;
    cmd.arg("--pattern")
        .arg("test")
        .arg("--filepath")
        .arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("test\nAnother test"));

    Ok(())
}
