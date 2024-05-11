use crate::{errors::LexError, token::Token};
use std::{iter::Peekable, str::Chars};

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    line: i16,
}

impl<'a> Lexer<'a> {
    /// Parse "key":"value".
    fn parse_kv(&mut self) -> Result<Vec<Token>, LexError> {
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
            Err(LexError::InvalidSyntax {
                line: self.line,
                expected: String::from(":"),
            })?
        };

        // Parse Value.
        if let Some(char) = self.read() {
            match char {
                '"' => tokens.push(self.read_string()?),
                'n' => tokens.push(self.read_null()?),
                't' => tokens.push(self.read_boolean_true()?),
                'f' => tokens.push(self.read_boolean_false()?),
                '{' => {
                    tokens.push(Token::LBraces);
                    tokens.append(&mut self.parse_kv()?)
                }
                '[' => tokens.push(self.read_array()?),
                c => {
                    if is_number(c) {
                        tokens.push(self.read_number(c)?);
                    } else {
                        Err(LexError::InvalidSyntax {
                            line: self.line,
                            expected: String::from("value"),
                        })?
                    }
                }
            }
        };
        Ok(tokens)
    }

    fn read_keyword(
        &mut self,
        mut buf: String,
        stop_at: char,
        expected_token: Token,
    ) -> Result<Token, LexError> {
        let mut ended = false;

        for c in self.input.by_ref() {
            if c == stop_at {
                ended = true;
                break;
            } else {
                buf.push(c)
            }
        }

        if !ended {
            Err(LexError::InvalidSyntax {
                line: self.line,
                expected: String::from(","),
            })?
        }

        if buf != expected_token.literal() {
            Err(LexError::InvalidKeyword {
                line: self.line,
                keyword: buf,
                maybe: expected_token.literal(),
            })?
        }

        Ok(expected_token)
    }

    /// Read string between double quotes.
    ///
    /// ## Errors
    /// - If string is not terminated.
    /// - If `:` is encountered before closing quote.
    fn read_string(&mut self) -> Result<Token, LexError> {
        let mut buf = String::new();
        let mut ended = false;

        for c in self.input.by_ref() {
            match c {
                '"' => {
                    ended = true;
                    break;
                }
                ':' => Err(LexError::MissingClosingQuote { line: self.line })?,
                ch => buf.push(ch),
            }
        }

        if !ended {
            Err(LexError::UnterminatedString { line: self.line })?
        }
        Ok(Token::Literal(buf))
    }

    fn read_number(&mut self, initial_char: char) -> Result<Token, LexError> {
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
            Err(LexError::UnterminatedNumber { line: self.line })?
        }

        // TODO handle parse error
        Ok(Token::Number(s.parse::<usize>().unwrap()))
    }

    fn read_null(&mut self) -> Result<Token, LexError> {
        self.read_keyword(String::from("n"), ',', Token::Null)
    }

    fn read_boolean_true(&mut self) -> Result<Token, LexError> {
        self.read_keyword(String::from("t"), ',', Token::True)
    }

    fn read_boolean_false(&mut self) -> Result<Token, LexError> {
        self.read_keyword(String::from("f"), ',', Token::False)
    }

    fn read_array(&mut self) -> Result<Token, LexError> {
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
            Err(LexError::UnterminatedArray { line: self.line })?
        }

        Ok(Token::Array)
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: Chars<'a>) -> Self {
        Lexer {
            input: input.peekable(),
            line: 1,
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::<Token>::new();

        // File is empty.
        if self.peek().is_none() {
            Err(LexError::EmptyFile)?
        }

        while let Some(char) = self.read() {
            match char {
                '{' => tokens.push(Token::LBraces),
                '}' => {
                    if tokens[tokens.len() - 1] == Token::Coma {
                        Err(LexError::TrailingComa { line: self.line })?
                    }
                    tokens.push(Token::RBraces);
                }
                '"' => tokens.append(&mut self.parse_kv()?),
                ',' => tokens.push(Token::Coma),
                c => Err(LexError::UnexpectedToken {
                    line: self.line,
                    token: c,
                })?,
            }
        }

        // Ensure proper start.
        if tokens[0] != Token::LBraces {
            Err(LexError::OpeningCurly)?
        }

        // Ensure proper ending.
        if tokens[tokens.len() - 1] != Token::RBraces {
            Err(LexError::ClosingCurly)?
        }

        Ok(tokens)
    }

    /// Strip newline and whitespace and return the next valid char.
    fn read(&mut self) -> Option<char> {
        loop {
            if let Some(ch) = self.peek() {
                if is_newline(ch) {
                    self.line += 1;
                    self.input.next();
                    continue;
                }
                if is_whitespace(ch) {
                    self.input.next();
                    continue;
                }
                break;
            }
            break;
        }

        self.input.next()
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
