use crate::yanrlox::token::Token;
use crate::yanrlox::error::Error;
use crate::yanrlox::token::TokenType;
use crate::yanrlox::token::TokenValue;

pub struct Scanner {
    stream: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    column: usize

}

impl Scanner {
    pub fn new(context: &str) -> Self {
        Scanner{
            stream: context.chars().collect(), 
            tokens: vec![], 
            start: 0, 
            current: 0, 
            line: 1, 
            column: 1
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Error> {
        
        while !self.at_end() {
            self.start = self.current;
            println!("tokens: {:?}", self.tokens);
            match self.scan_token() {
                Ok(()) => {},
                Err(e) => return Err(e)
            }
            println!("tokens: {:?}", self.tokens);
        }

        self.push(TokenType::Eof);

        Ok(self.tokens.clone())
    }

    pub fn scan_token(&mut self) -> Result<(), Error> {
        let c = self.advance();
        println!("st |{}|", c);
        match c {
            '(' => self.push(TokenType::LeftParen),
            ')' => self.push(TokenType::RightParen),
            '{' => self.push(TokenType::LeftBrace),
            '}' => self.push(TokenType::RightBrace),
            ',' => self.push(TokenType::Comma),
            '.' => self.push(TokenType::Dot),
            '-' => self.push(TokenType::Minus),
            '+' => self.push(TokenType::Plus),
            ';' => self.push(TokenType::Semicolon),
            '*' => self.push(TokenType::Star),

            '!' => self.check_combo('=', TokenType::Bang, TokenType::BangEqual),
            '=' => self.check_combo('=', TokenType::Equal, TokenType::EqualEqual),
            '<' => self.check_combo('=', TokenType::Less, TokenType::LessEqual),
            '>' => self.check_combo('=', TokenType::Greater, TokenType::GreaterEqual),

            '/' => {
                if self.match_next('/') {
                    while !self.at_end() {
                        if self.peek() == '\n' {
                            self.column = 0;
                            self.line += 1;
                            break;
                        }
                        self.advance(); 
                    }
                    println!("over at {}", self.at_end());
                } else {
                    self.push(TokenType::Slash);
                }
            }

            ' ' => self.column += 1,
            '\n' => {
                self.column = 1;
                self.line += 1;
            },
            '\r' | '\t' => {},

            '\"' => match self.string() {
                Ok(()) => {},
                Err(e) => return Err(e)
            }


            _ => return Err(Error::pos_error(&format!("Unexpected character: |{}|", c), self.line, self.column, "A charater not in the standard definition of lox has be detected"))

        };

        Ok(())
    }

    fn string(&mut self) -> Result<(), Error> {
        while self.peek() != '\"' && !self.at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            } 
            self.advance();
        }

        if self.at_end() {
            return Err(Error::pos_error("Unterminated String", self.line, self.column, "A string value was started but was never ended via \""))
        }

        self.advance(); //closing "

        let val: String = (self.stream[self.start+1..self.current-1]).iter().collect();

        self.tokens.push(Token::add_val(TokenType::String, TokenValue::String(val), 
        self.line, self.column, self.current-self.start));

        Ok(())
    }

    fn push(&mut self, t: TokenType) {
        self.tokens.push(Token::add_type(t, self.line, self.column, self.current-self.start));
    }

    fn advance(&mut self) -> char {
        let c = self.stream[self.current];
        self.current += 1;
        self.column += 1;
        return c;
    }

    fn peek(&self) -> char {
        if self.at_end() {
            '\0'
        } else {
            self.stream[self.current]
        }
    }

    fn check_combo(&mut self, c: char, one: TokenType, two: TokenType) {
        if self.match_next(c) {
            self.push(two);
        } else {
            self.push(one);
        }
    }

    fn match_next(&mut self, c: char) -> bool {
        if self.at_end() || self.stream[self.current] != c {
            return false
        }
        self.current += 1;
        self.column += 1;
        return true
    }

    fn at_end(&self) -> bool {
        self.current >= self.stream.len()
    }
}
