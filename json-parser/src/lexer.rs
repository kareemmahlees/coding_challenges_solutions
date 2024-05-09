use std::str::Chars;

use anyhow::Result;

#[derive(Debug)]
pub enum Token {
    Lbraces,
    Rbraces,
    Eof,
}

impl Token {
    fn literal(&self) -> Option<char> {
        match self {
            Token::Lbraces => Some('{'),
            Token::Rbraces => Some('}'),
            Token::Eof => None,
        }
    }
}

pub struct Lexer<'a> {
    input: Chars<'a>,
    cur: usize,
    line: i16,
}

impl<'a> Lexer<'a> {
    pub fn new(input: Chars<'a>) -> Self {
        Lexer {
            input,
            cur: 0,
            line: 1,
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::<Token>::new();
        let iter_len = self.input.to_owned().count();

        // File is empty
        if iter_len == 0 {
            anyhow::bail!("Empty JSON file is invalid, expected '{{}}'")
        }

        while let Some(char) = self.read() {
            // We are at the beginning of the file
            if self.cur == 0 {
                dbg!(char);
                anyhow::ensure!(
                    char == Token::Lbraces.literal().unwrap(),
                    "Expected opening braces, found: {}",
                    char
                )
            }

            // We are at the end of the file
            if self.cur == iter_len {
                anyhow::ensure!(
                    char == Token::Rbraces.literal().unwrap(),
                    "Expected ending braces, found: {}",
                    char
                );
            }

            if is_newline(char) {
                self.eat_newline();
                continue;
            }
            if is_whitespace(char) {
                self.eat_whitespace();
                continue;
            }
            match char {
                '{' => tokens.push(Token::Lbraces),
                '}' => tokens.push(Token::Rbraces),
                c => {
                    anyhow::bail!("Unexpected token: {} at line {}", c, self.line)
                }
            }
        }

        tokens.push(Token::Eof);

        Ok(tokens)
    }

    /// Get the current char..
    fn read(&mut self) -> Option<char> {
        let next = self.input.next();
        self.cur += 1;
        next
    }

    /// If we jumped into a new line, increase line number and consume char without tokenization.
    fn eat_newline(&mut self) {
        self.line += 1;
        self.input.next();
    }

    /// Consume the whitespace without tokenization.
    fn eat_whitespace(&mut self) {
        self.input.next();
    }
}

/// Check if the char is '\n' or '\r'.
fn is_newline(char: char) -> bool {
    char == '\n' || char == '\r'
}

/// Check if char is a whitespace.
fn is_whitespace(char: char) -> bool {
    char == ' '
}
