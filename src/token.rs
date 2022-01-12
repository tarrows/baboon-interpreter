#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
  ILLEGAL,

  // End of File
  EOF,

  // Identifiers and Literals
  IDENT(String),
  INT(i32),

  // Operators
  ASSIGN,
  PLUS,

  // Delimiters
  COMMA,
  SEMICOLON,

  // Parentheses
  LPAREN,
  RPAREN,
  LBRACE,
  RBRACE,

  // Keywords
  FUNCTION,
  LET,
}

impl Token {
  pub fn keyword(ident: &str) -> Option<Token> {
    match ident {
      "fn"  => Some(Token::FUNCTION),
      "let" => Some(Token::LET),
      _     => None
    }
  }
}
