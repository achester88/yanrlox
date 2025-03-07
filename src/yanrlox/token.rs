use std::fmt;

#[derive(Debug)]
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

  Eof
}

#[derive(Debug)]
pub enum TokenValue {
    Bool(bool),
    String(String),
    Number(f64)
}

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
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.token_type {
            TokenType::Number => write!(f, "[Number: {:?}]", &self.val),
            _ => write!(f, "[{:?}]", &self.token_type)
        }
        //write!(f, "Token")
    }
}
