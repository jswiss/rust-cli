use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::io::Write;
use std::process::Command; // Run programs
use tempfile::NamedTempFile; // has null

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(
        file,
        "A test \n Actual Content \n more content \n more tests"
    )?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A test \n more tests"));

    Ok(())
}

// #[test]
// fn empty_string_pattern() -> Result<(), Box<dyn std::error::Error>> {
//     let mut cmd = Command::cargo_bin("grrs")?;
//     let mut file = NamedTempFile::new()?;
//     writeln!(file, "This is the file\nIt is a file\nblah blah blah")?;

//     let null_arg: *const {}= ptr::null();

//     cmd.arg(null_arg).arg(file.path());
//     cmd.assert().failure().stderr(predicate::str::contains(
//         "The following required arguments were not provided",
//     ));

//     Ok(())
// }
