use std::str::Chars;
use crate::token::{ Token, TokenType };
use crate::keywords::KEYWORDS;

pub struct Lexer<'a> {
    source: Chars<'a>,
    lookahead: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a String) -> Self {
        let mut source_iter = source.chars();

        Self {
            lookahead: source_iter.next(),
            source: source_iter,
        }
    }

    pub fn lex(&mut self, valid_toks: &[TokenType]) -> Result<Token, Token> {
        use TokenType::*;

        loop {
            if self.check_for(&['\r', '\t', ' ', '\n']) {
                self.advance();
            }
            else {
                break
            }
        }

        let token = {
            if self.check_if(is_name_start) {
                self.lex_name()
            }
            else if self.check_if(is_digit) {
                self.lex_number()
            }
            else if self.check('"') {
                self.lex_string()
            }
            else if self.check('!') {
                self.advance();
                Token::Keyword(EXCL)
            }
            else if self.check('(') {
                self.advance();
                Token::Keyword(LPAREN)
            }
            else if self.check(')') {
                self.advance();
                Token::Keyword(RPAREN)
            }
            else if self.check('{') {
                self.advance();
                Token::Keyword(LBRACE)
            }
            else if self.check('}') {
                self.advance();
                Token::Keyword(RBRACE)
            }
            else if self.is_at_end() {
                Token::Eof
            }
            else {
                panic!("LexerError: Unexpected character '{}'", self.lookahead.unwrap())
            }
        };

        if valid_toks.contains(&token.typ()) {
            println!("{:?}", token);
            Ok(token)
        }
        else {
            println!("error: {:?}", token);
            Err(token)
        }
    }

    fn lex_name(&mut self) -> Token {
        let mut name = String::new();

        while self.check_if(is_name) {
            if let Some(c) = self.lookahead {
                name.push(c);
            }

            self.advance();
        }

        if let Some(tok_type) = KEYWORDS.get(name.as_str()).cloned() {
            Token::Keyword(tok_type)
        } else {
            Token::Name(name)
        }
    }

    fn lex_number(&mut self) -> Token {
        let mut number_string = String::new();

        while self.check_if(is_digit) {
            if let Some(c) = self.lookahead {
                number_string.push(c);
            }

            self.advance();
        }

        let number = number_string.parse::<i32>().unwrap();

        Token::Number(number)
    }

    fn lex_string(&mut self) -> Token {
        let mut string = String::new();

        self.advance();

        while !self.check('"') {
            if let Some(c) = self.lookahead {
                string.push(c);
            }

            self.advance();
        }

        self.advance();

        Token::String(string)
    }

    fn check_for(&self, checkings: &[char]) -> bool
    { checkings.iter().any(|c| { self.check(*c) }) }

    fn check(&self, checking: char) -> bool
    { self.check_if( |c| c == checking ) }

    fn check_if(&self, predicate: impl Fn(char) -> bool) -> bool
    { self.lookahead.map_or(false, predicate) }

    fn advance(&mut self)
    { self.lookahead = self.source.next() }

    fn is_at_end(&self) -> bool
    { self.lookahead.is_none() }
}

fn is_name_start(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '@' | '$' | '%' | ':'
        | '^' | '&' | '*' | '-' | '+' | '=' | '|' | ';'
        | '~' | '<' | '>' | '/' | '?' | '.' | ',' | '\\' => true,
        _ => false,
    }
}

fn is_digit(c: char) -> bool {
    match c {
        '0'..='9' => true,
        _ => false,
    }
}

fn is_name(c: char) -> bool {
    is_name_start(c) || is_digit(c) || c == '\''
}
