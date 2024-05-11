use std::{iter::Peekable, str::Chars};

use anyhow::{Ok, Result};

#[derive(Debug, PartialEq)]
pub enum Token {
    LBraces,
    RBraces,
    Literal(String),
    Number(usize),
    Object,
    Array,
    Colon,
    Coma,
    Null,
    True,
    False,
}

impl Token {
    fn literal(&self) -> String {
        match self {
            Token::LBraces => String::from("{"),
            Token::RBraces => String::from("}"),
            Token::Literal(s) => s.clone(),
            Token::Number(n) => n.to_string(),
            Token::Object => String::from("{{}}"),
            Token::Array => String::from("[]"),
            Token::Colon => String::from(":"),
            Token::Coma => String::from(","),
            Token::Null => String::from("null"),
            Token::True => String::from("true"),
            Token::False => String::from("false"),
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

    /// Parse "key":"value".
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
                '"' => tokens.push(self.read_string()?),
                'n' => tokens.push(self.read_null()?),
                't' => tokens.push(self.read_boolean_true()?),
                'f' => tokens.push(self.read_boolean_false()?),
                '{' => tokens.push(self.read_object()?),
                '[' => tokens.push(self.read_array()?),
                c => {
                    if is_number(c) {
                        tokens.push(self.read_number(c)?);
                    } else {
                        anyhow::bail!(
                            "Invalid expression at line {}, Expected 'value'.",
                            self.line
                        );
                    }
                }
            }
        };
        Ok(tokens)
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

    fn read_number(&mut self, initial_char: char) -> Result<Token> {
        let mut s = String::from(initial_char);
        let mut ended = false;

        for c in self.input.by_ref() {
            if is_number(c) {
                s.push(c);
            } else {
                ended = true;
                break;
            }
        }

        if !ended {
            anyhow::bail!("Unterminated number at line: {}", self.line);
        }

        // TODO handle parse error
        Ok(Token::Number(s.parse::<usize>().unwrap()))
    }

    fn read_null(&mut self) -> Result<Token> {
        // we start with 'n' because we are already at it and any further read will advance the cursor away from it.
        let mut null = String::from("n");
        let mut ended = false;

        for c in self.input.by_ref() {
            match c {
                ',' => {
                    ended = true;
                    break;
                }
                ch => null.push(ch),
            }
        }

        if !ended {
            anyhow::bail!("Unterminated Identifier, expected comma")
        }

        anyhow::ensure!(
            null == Token::Null.literal(),
            "Invalid identifier: {}, Maybe you mean 'null'?",
            null
        );

        Ok(Token::Null)
    }

    fn read_boolean_true(&mut self) -> Result<Token> {
        // we start with 'n' because we are already at it and any further read will advance the cursor away from it.
        let mut true_literal = String::from("t");
        let mut ended = false;

        for c in self.input.by_ref() {
            match c {
                ',' => {
                    ended = true;
                    break;
                }
                ch => true_literal.push(ch),
            }
        }

        if !ended {
            anyhow::bail!("Unterminated Identifier, expected comma")
        }

        anyhow::ensure!(
            true_literal == Token::True.literal(),
            "Invalid identifier: {}, Maybe you mean 'true'?",
            true_literal
        );

        Ok(Token::True)
    }

    fn read_boolean_false(&mut self) -> Result<Token> {
        // we start with 'n' because we are already at it and any further read will advance the cursor away from it.
        let mut false_literal = String::from("f");
        let mut ended = false;

        for c in self.input.by_ref() {
            match c {
                ',' => {
                    ended = true;
                    break;
                }
                ch => false_literal.push(ch),
            }
        }

        if !ended {
            anyhow::bail!("Unterminated Identifier, expected comma")
        }

        anyhow::ensure!(
            false_literal == Token::False.literal(),
            "Invalid identifier: {}, Maybe you mean 'false'?",
            false_literal
        );

        Ok(Token::False)
    }

    fn read_object(&mut self) -> Result<Token> {
        let mut ended = true;

        for ch in self.input.by_ref() {
            match ch {
                '}' => {
                    ended = true;
                    break;
                }
                _ => todo!(),
            }
        }

        if !ended {
            anyhow::bail!("Unterminated object, expected '}}'");
        }

        Ok(Token::Object)
    }

    fn read_array(&mut self) -> Result<Token> {
        let mut ended = true;

        for ch in self.input.by_ref() {
            match ch {
                ']' => {
                    ended = true;
                    break;
                }
                _ => todo!(),
            }
        }

        if !ended {
            anyhow::bail!("Unterminated array, expected ']'");
        }

        Ok(Token::Array)
    }

    /// Peak ahead into the input.
    fn peek(&mut self) -> Option<char> {
        self.input.peek().copied()
    }
}

/// Check if the char is '\n' or '\r'.
fn is_newline(char: char) -> bool {
    char == '\n' || char == '\r'
}

fn is_whitespace(char: char) -> bool {
    char == ' '
}

fn is_number(char: char) -> bool {
    ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'].contains(&char)
}
