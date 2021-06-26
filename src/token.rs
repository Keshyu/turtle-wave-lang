#[derive(Debug)]
pub enum Token {
    Keyword(TokenType),
    Name(String),
    Number(i32),
    String(String),
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    NEST, OF, LEATHERBACK, GREEN, TURTLES,
    SHORELINE, HORIZON,
    NAME, NUMBER, STRING,
    EXCL, LBRACE, RBRACE, LPAREN, RPAREN,
    EOF,
}

impl Token {
    pub fn typ(&self) -> &TokenType {
        use Token::*;
        use TokenType::*;

        match self {
            Keyword(ttype) => &ttype,
            Name(_) => &NAME,
            String(_) => &STRING,
            Number(_) => &NUMBER,
            Eof => &EOF,
        }
    }

    pub fn unwrap_name(self) -> String {
        match self {
            Self::Name(name) => name,
            _ => panic!("Attempt to unwrap_name on a non-name value"),
        }
    }

    pub fn unwrap_number(self) -> i32 {
        match self {
            Self::Number(number) => number,
            _ => panic!("Attempt to unwrap_number on a non-number value"),
        }
    }

    pub fn unwrap_string(self) -> String {
        match self {
            Self::String(string) => string,
            _ => panic!("Attempt to unwrap_string on a non-string value"),
        }
    }
}
