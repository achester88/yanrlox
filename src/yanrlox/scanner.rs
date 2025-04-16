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

    /*pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Error> {
        
        while !self.at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(()) => {},
                Err(e) => return Err(e)
            }
        }

        self.push(TokenType::Eof);

        Ok(self.tokens.clone())
    }*/

    pub fn scan_token(&mut self) -> Result<Token, Error> {
        if self.at_end() {
            return Ok(self.create(TokenType::Eof))
        }
        let c = self.advance();
        let next_token = match c {
            '(' => self.create(TokenType::LeftParen),
            ')' => self.create(TokenType::RightParen),
            '{' => self.create(TokenType::LeftBrace),
            '}' => self.create(TokenType::RightBrace),
            ',' => self.create(TokenType::Comma),
            '.' => self.create(TokenType::Dot),
            '-' => self.create(TokenType::Minus),
            '+' => self.create(TokenType::Plus),
            ';' => self.create(TokenType::Semicolon),
            '*' => self.create(TokenType::Star),

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

                    self.start = self.current;
                    match self.scan_token() {
                        Ok(t) => t,
                        Err(e) => return Err(e)
                    }
                } else {
                     self.create(TokenType::Slash)
                }
            }

            ' ' => {
                self.column += 1;

                self.start = self.current;
                match self.scan_token() {
                    Ok(t) => t,
                    Err(e) => return Err(e)
                }
            },
            '\n' => {
                self.column = 1;
                self.line += 1;

                self.start = self.current;
                match self.scan_token() {
                    Ok(t) => t,
                    Err(e) => return Err(e)
                }
            },
            '\r' | '\t' => {
                self.start = self.current;
                match self.scan_token() {
                    Ok(t) => t,
                    Err(e) => return Err(e)
                }
            },

            '\"' => match self.string() {
                Ok(()) => {
                    self.start = self.current;
                    match self.scan_token() {
                        Ok(t) => t,
                        Err(e) => return Err(e)
                    }
                }
                ,
                Err(e) => return Err(e)
            },
            
            _ => {
                if c.is_digit(10) {
                    self.number()
                } else if c.is_alphabetic() || c == '_' {
                    self.identifier()
                } else {
                return Err(Error::pos_error(&format!("Unexpected character: |{}|", c), self.line, self.column, "A charater not in the standard definition of lox has be detected"));
                }
            }

        };

        self.start = self.current;
        Ok(next_token)

    }

    fn identifier(&mut self) -> Token {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let val: String = (self.stream[self.start..self.current]).iter().collect();

        let token_type = match val.as_ref() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "fun" => TokenType::Fun,
            "for" => TokenType::For,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print, 
            "return" => TokenType::Return,
            "super" => TokenType::Super, 
            "ths" => TokenType::This, 
            "true" => TokenType::True, 
            "var" => TokenType::Var, 
            "while" => TokenType::While,

            _ => TokenType::Identifier
        };

        if token_type == TokenType::Identifier {
            return (Token::add_val(token_type, TokenValue::Name(val),
            self.line, self.column, self.current-self.start));
        } else {
            return self.create(token_type);
        }
    }

    fn number(&mut self) -> Token {
        while self.peek().is_digit(10) {
            self.advance();
        }
        
        let next = self.current + 1;
        if self.peek() == '.' && next < self.stream.len() && self.stream[next].is_digit(10) {
            self.advance(); //.
            
            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let val: String = (self.stream[self.start..self.current]).iter().collect();
        return (Token::add_val(TokenType::Number, TokenValue::Number(val.parse::<f64>().unwrap()), 
        self.line, self.column, self.current-self.start));
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

    fn create(&mut self, t: TokenType) -> Token {
        Token::add_type(t, self.line, self.column, self.current-self.start)
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

    fn check_combo(&mut self, c: char, one: TokenType, two: TokenType) -> Token {
        if self.match_next(c) {
            return self.create(two)
        } else {
            return self.create(one)
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

    pub fn scan_next_token(&mut self) -> Result<Token, Error> {
        return self.scan_token()
    }
}

