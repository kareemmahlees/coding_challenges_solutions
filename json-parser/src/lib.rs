mod errors;
mod lexer;
mod token;

use anyhow::{Context, Result};
use std::io::Read;

pub fn run() -> Result<()> {
    let file_path = std::env::args().nth(1).context("get file_path")?;
    let contents = read_file_contents(&file_path)?;

    let mut lexer = lexer::Lexer::new(contents.chars());
    let tokens = lexer.lex()?;

    dbg!(tokens);

    Ok(())
}

fn read_file_contents(file_path: &str) -> Result<String> {
    let mut file = std::fs::File::open(file_path).context("open json file")?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .context("read file contents")?;

    Ok(contents)
}
