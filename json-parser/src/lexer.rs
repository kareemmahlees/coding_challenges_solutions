use std::{iter::Peekable, str::Chars};

use anyhow::Result;

#[derive(Debug, PartialEq)]
pub enum Token {
    LBraces,
    RBraces,
    Literal(String),
    Colon,
    Coma,
}

impl Token {
    fn literal(&self) -> String {
        match self {
            Token::LBraces => String::from("{"),
            Token::RBraces => String::from("}"),
            Token::Literal(s) => s.clone(),
            Token::Colon => String::from(":"),
            Token::Coma => String::from(","),
        }
    }
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

        // File is empty.
        if self.peek().is_none() {
            anyhow::bail!("Empty JSON file is invalid, expected '{{}}'")
        }

        while let Some(char) = self.read() {
            match char {
                '{' => tokens.push(Token::LBraces),
                '}' => {
                    anyhow::ensure!(tokens[tokens.len() - 1] != Token::Coma, "Trailing Coma");
                    tokens.push(Token::RBraces);
                }
                '"' => tokens.append(&mut self.parse_kv()?),
                ',' => tokens.push(Token::Coma),
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

    /// Strip newline and whitespace and return the next valid char.
    fn read(&mut self) -> Option<char> {
        let mut ch: Option<char> = None;

        for char in self.input.by_ref() {
            if is_newline(char) {
                self.line += 1;
                continue;
            } else if is_whitespace(char) {
                continue;
            } else {
                ch = Some(char);
                break;
            }
        }
        ch
    }

    /// Peak ahead into the input.
    fn peek(&mut self) -> Option<char> {
        self.input.peek().copied()
    }

    /// Read string between double quotes.
    ///
    /// ## Errors
    /// - If string is not terminated.
    /// - If `:` is encountered before closing quote.
    fn read_string(&mut self) -> Result<Token> {
        let mut s = String::new();
        let mut ended = false;

        for c in self.input.by_ref() {
            match c {
                '"' => {
                    ended = true;
                    break;
                }
                ':' => anyhow::bail!("Expected closing quote "),
                ch => s.push(ch),
            }
        }

        if !ended {
            anyhow::bail!("Unterminated string at line: {}", self.line);
        }
        Ok(Token::Literal(s))
    }

    /// Prase "key":"value".
    fn parse_kv(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::<Token>::new();

        // Parse key.
        let key = self.read_string()?;
        tokens.push(key);

        // Parse Colon.
        if self
            .read()
            .is_some_and(|c| c.to_string() == Token::Colon.literal())
        {
            tokens.push(Token::Colon);
        } else {
            anyhow::bail!("Invalid expression at line {}, Expected ':'.", self.line)
        };

        // Parse Value.
        if let Some(char) = self.read() {
            match char {
                '"' => {
                    let literal = self.read_string()?;
                    tokens.push(literal);
                }
                // parse other data types
                _ => todo!(),
            };
        } else {
            anyhow::bail!(
                "Invalid expression at line {}, Expected 'value'.",
                self.line
            )
        }
        Ok(tokens)
    }
}

/// Check if the char is '\n' or '\r'.
fn is_newline(char: char) -> bool {
    char == '\n' || char == '\r'
}

fn is_whitespace(char: char) -> bool {
    char == ' '
}
