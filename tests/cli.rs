use std::process::Command;  // Run programs
use assert_cmd::prelude::*; // Add methods on commands

use tempfile::tempdir;
use std::fs::{File, create_dir, read_to_string};
use std::io::Write;
use std::path::PathBuf;

fn create(path: PathBuf, contents: String) -> File {
    let mut file = File::create(path).expect("failed to create file");
    writeln!(file, "{}", contents).expect("Failed to write to file");

    file
}

#[test]
fn generates_summary_md() -> Result<(), Box<std::error::Error>> {
    let dir = tempdir()?;
    let path = dir.path();

    println!("{}", path.display());

    create_dir(path.join("subdir"))?; // this'll have a README.md in it.
    create_dir(path.join("other_dir"))?; // this won't.
    // This'll produce weird behavior, until I get around to fixing it.

    let flat_file = create(
        path.join("flat_file.md"),
        "\n# A Thing\n\n ## Contents".to_string()
    );

    let subdir_readme = create(
        path.join("subdir/README.md"),
        "\n\n# Subdirectory".to_string()
    );
    let subdir_other = create(
        path.join("subdir/other.md"),
        "# Other".to_string()
    );

    let other = create(
        path.join("other_dir/some_content.md"),
        "## Whatever\n\n # Moar Content".to_string()
    );

    let mut cmd = Command::main_binary()?;
    cmd.arg(path);
    cmd.assert()
        .success()
        .stdout("")
        .stderr("");

    let received_summary = read_to_string(path.join("SUMMARY.md"))
        .expect("failed to read SUMMARY.md");

    assert_eq!(
        received_summary,
        "# https://github.com/rust-lang-nursery/mdBook/issues/677
- [A Thing](flat_file.md)
    - [Moar Content](other_dir/some_content.md)
- [Subdirectory](subdir/README.md)
    - [Other](subdir/other.md)
"
    );

    drop(flat_file);
    drop(subdir_readme);
    drop(subdir_other);
    drop(other);

    dir.close()?;
    Ok(())
}

//#[test]
//fn file_doesnt_exist() -> Result<(), Box<std::error::Error>> {
//    let mut cmd = Command::main_binary()?;
//    cmd.arg("foobar")
//        .arg("test/file/doesnt/exist");
//    cmd.assert()
//        .failure()
//        .stderr(predicate::str::contains("No such file or directory"));
//
//    Ok(())
//}