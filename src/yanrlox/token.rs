use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
  LeftParen, RightParen, LeftBrace, RightBrace,
  Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

  // One or two character tokens.
  Bang, BangEqual,
  Equal, EqualEqual,
  Greater, GreaterEqual,
  Less, LessEqual,

  // Literals.
  Identifier, String, Number,

  // Keywords.
  And, Class, Else, False, Fun, For, If, Nil, Or,
  Print, Return, Super, This, True, Var, While,

  Eof, Error
}

#[derive(Debug, Clone)]
pub enum TokenValue {
    String(String),
    Number(f64),
    Name(String)
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub val: Option<TokenValue>,
    pub line: usize,
    pub column: usize,
    length: usize
}

impl Token {
    pub fn add_type(t: TokenType, line: usize, column: usize, length: usize) -> Self {
        Token{token_type: t, val: None, line: line, column: column, length: length}
    }

    pub fn add_val(t: TokenType, val: TokenValue, line: usize, column: usize, length: usize) -> Self {
        Token{token_type: t, val: Some(val), line: line, column: column, length: length}
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.token_type {
            TokenType::Number => write!(f, "[Number: {:?}]", &self.val.clone().unwrap()),
            TokenType::String => write!(f, "[String: {:?}]", &self.val.clone().unwrap()),
            TokenType::Identifier => write!(f, "[Identifier: {:?}]", &self.val.clone().unwrap()),
            _ => write!(f, "[{:?}]", &self.token_type)
        }
        //write!(f, "Token")
    }
}
