use assert_cmd::cargo::*;
use predicates::prelude::*;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("termgloss");

    cmd.arg("tests/input.typ").arg("-o").arg("-");
    cmd.assert()
        .success()
        .stdout(predicate::str::diff(include_str!("expected.typ")));

    Ok(())
}
