use std::{iter::Peekable, str::Chars};

use anyhow::Result;

#[derive(Debug, PartialEq)]
pub enum Token {
    LBraces,
    RBraces,
}

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    line: i16,
}

impl<'a> Lexer<'a> {
    pub fn new(input: Chars<'a>) -> Self {
        Lexer {
            input: input.peekable(),
            line: 1,
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::<Token>::new();

        // File is empty
        if self.peek().is_none() {
            anyhow::bail!("Empty JSON file is invalid, expected '{{}}'")
        }

        while let Some(char) = self.read() {
            if is_newline(char) {
                self.eat_newline();
                continue;
            }
            if is_whitespace(char) {
                self.eat_whitespace();
                continue;
            }
            match char {
                '{' => tokens.push(Token::LBraces),
                '}' => tokens.push(Token::RBraces),
                c => {
                    anyhow::bail!("Unexpected token: {} at line {}", c, self.line)
                }
            }
        }

        // Ensure proper start.
        if tokens[0] != Token::LBraces {
            anyhow::bail!("Expected opening braces")
        }

        // Ensure proper ending.
        if tokens[tokens.len() - 1] != Token::RBraces {
            anyhow::bail!("Expected ending braces");
        }

        Ok(tokens)
    }

    /// Get the current char.
    fn read(&mut self) -> Option<char> {
        self.input.next()
    }

    /// Peak ahead into the input.
    fn peek(&mut self) -> Option<char> {
        self.input.peek().copied()
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
