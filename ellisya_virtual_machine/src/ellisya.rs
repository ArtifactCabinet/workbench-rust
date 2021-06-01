use std::fs;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    fs::write("foo.txt", "Lorem ipsum")?;
    Ok(())
}