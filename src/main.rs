use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

fn append_line_to_schema(path: &str, line: &str) -> std::io::Result<()> {
    let path = Path::new(path).join("schema.yml");

    let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(true)
                    .open(&path)?;

    writeln!(file, "{}", line)?;

    Ok(())
}

fn main() {
    let path = "/Users/sung/Desktop/git_repos/dbt-typer/tests/test_dbt_project/models";
    let line = "hello world";

    match append_line_to_schema(path, line) {
        Err(e) => eprintln!("Error writing to file: {}", e),
        _ => (),
    }
}