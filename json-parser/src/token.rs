#[derive(Debug, PartialEq)]
pub enum Token {
    LBraces,
    RBraces,
    Literal(String),
    Number(usize),
    Array,
    Colon,
    Coma,
    Null,
    True,
    False,
}

impl Token {
    pub(crate) fn literal(&self) -> String {
        match self {
            Token::LBraces => String::from("{"),
            Token::RBraces => String::from("}"),
            Token::Literal(s) => s.clone(),
            Token::Number(n) => n.to_string(),
            Token::Array => String::from("[]"),
            Token::Colon => String::from(":"),
            Token::Coma => String::from(","),
            Token::Null => String::from("null"),
            Token::True => String::from("true"),
            Token::False => String::from("false"),
        }
    }
}
