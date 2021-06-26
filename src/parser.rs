use crate::lexer::Lexer;
use crate::token::Token;
use crate::token::TokenType::{ self, * };
use crate::syntax_tree::*;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    cache_tok: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a String) -> Self {
        Self {
            lexer: Lexer::new(source),
            cache_tok: None,
        }
    }

    pub fn parse(&mut self) -> ProgramIsland {
        // program : nest* SHORELINE flow HORIZON EOF
        let mut nests = Vec::new();

        let next_tok = self.lex(&[NEST, SHORELINE]).typ();

        if next_tok == &NEST {
            let nest = self.parse_nest();
            nests.push(nest);
        }

        self.eat(&[SHORELINE]);

        let sea_flow = Flow(self.parse_bare_flow(HORIZON));

        self.eat(&[HORIZON]);
        self.eat(&[EOF]);

        ProgramIsland { nests, sea_flow }
    }

    fn parse_nest(&mut self) -> Nest {
        // nest : NEST OF GREEN TURTLES LPAREN NAME* RPAREN
        let mut green_turtles = Vec::new();

        self.eat(&[NEST]);
        self.eat(&[OF]);
        self.eat(&[GREEN]);
        self.eat(&[TURTLES]);
        self.eat(&[LPAREN]);

        let mut next_tok = self.lex(&[NAME, RPAREN]).typ();

        while next_tok != &RPAREN {
            let name = self.eat(&[NAME]).unwrap_name();
            green_turtles.push(name);

            next_tok = self.lex(&[NAME, RPAREN]).typ();
        }

        self.eat(&[RPAREN]);

        Nest::Green(green_turtles)
    }

    fn parse_bare_flow(&mut self, ending: TokenType) -> Vec<Value> {
        // flow : value*
        let mut values = Vec::new();

        let mut next_tok = self.lex(&[ending, NAME, NUMBER, STRING, EXCL, LPAREN, LBRACE]);

        while next_tok.typ() != &ending {
            let value = self.parse_value();
            values.extend(value);

            next_tok = self.lex(&[ending, NAME, NUMBER, STRING, EXCL, LPAREN, LBRACE]);
        }

        values
    }

    fn parse_value(&mut self) -> Vec<Value> {
        // value : simple_value
        //       | group
        //       | internal_flow
        let token = self.lex(&[NAME, NUMBER, STRING, EXCL, LPAREN, LBRACE]);

        match token.typ() {
            LPAREN => self.parse_group(),
            LBRACE => vec![self.parse_internal_flow()],
            _ => vec![self.parse_simple_value()],
        }
    }

    #[inline]
    fn parse_internal_flow(&mut self) -> Value {
        // internal_flow : LBRACE bare_flow RBRACE
        self.eat(&[LBRACE]);

        let values = self.parse_bare_flow(RBRACE);

        self.eat(&[RBRACE]);

        Value::InternalFlow(values)
    }

    #[inline]
    fn parse_simple_value(&mut self) -> Value {
        // simple_value : NAME | NUMBER | STRING | EXCL
        let token = self.eat(&[NAME, NUMBER, STRING, EXCL]);

        match token.typ() {
            NAME => Value::Name(token.unwrap_name()),
            NUMBER => Value::Number(token.unwrap_number()),
            STRING => Value::String(token.unwrap_string()),
            EXCL => Value::Trigger,
            _ => panic!("Impossible"),
        }
    }

    #[inline]
    fn parse_group(&mut self) -> Vec<Value> {
        // group : LPAREN bare_flow RPAREN
        self.eat(&[LPAREN]);

        let values = self.parse_bare_flow(RPAREN);

        self.eat(&[RPAREN]);



        values
    }

    fn lex(&mut self, valid_toks: &[TokenType]) -> &Token {
        match self.cache_tok {
            Some(_) => &self.cache_tok.as_ref().unwrap(),
            None => {
                self.cache_tok = Some(
                    self.lexer.lex(valid_toks).map_err(|token| {
                        self.error(valid_toks, token.typ())
                    }).unwrap()
                );
                &self.cache_tok.as_ref().unwrap()
            }
        }
    }

    fn eat(&mut self, valid_toks: &[TokenType]) -> Token {
        match self.cache_tok.take() {
            Some(token) => {
                // println!("{:?}", token);
                if valid_toks.contains(token.typ()) {
                    token
                }
                else {
                    self.error(valid_toks, token.typ())
                }
            }
            None => {
                self.lex(valid_toks);
                // println!("{:?}", self.cache_tok);
                self.cache_tok.take().unwrap()
            }
        }
    }

    #[inline]
    fn error(&self, expected_toks: &[TokenType], got_tok: &TokenType) -> ! {
        panic!(
            "Expected one of {:?}, but got {:?}",
            expected_toks, got_tok,
        )
    }
}
