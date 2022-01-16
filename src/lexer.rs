use crate::token::Token;

pub struct Lexer<'a> {
  input: std::str::Chars<'a>,
  curr: char,  // current character
  peek: char,  // next (peeked) character
}


impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    let mut lexer = Lexer {
      input: input.chars(),
      curr: '\u{0}',  // null
      peek: '\u{0}',
    };

    // set first input character to peek
    lexer.read_char();

    // set first input character to curr
    lexer.read_char();

    lexer
  }

  fn read_char(&mut self) -> char {
    let c = self.curr;

    self.curr = self.peek;
    self.peek = self.input.next().unwrap_or('\u{0}');

    c
  }

  pub fn next_token(&mut self) -> Token {
    self.skip_whitespaces();

    let token = match self.curr {
      '=' => Token::ASSIGN,
      ';' => Token::SEMICOLON,
      '(' => Token::LPAREN,
      ')' => Token::RPAREN,
      ',' => Token::COMMA,
      '+' => Token::PLUS,
      '{' => Token::LBRACE,
      '}' => Token::RBRACE,
      '\u{0}' => Token::EOF,
      c => {
        if is_letter(c) {
          // skip reading next char
          return self.read_identifier();
        } else if c.is_ascii_digit() {
          // skip reading next char
          return self.read_number();
        } else {
          Token::ILLEGAL
        }
      },
    };

    self.read_char();

    token
  }

  fn skip_whitespaces(&mut self) {
    while self.curr == ' ' || self.curr == '\t' || self.curr == '\n' || self.curr == '\r' {
      self.read_char();
    }
  }

  fn read_identifier(&mut self) -> Token {
    let mut ident = String::new();

    while is_letter(self.curr) {
      ident.push(self.read_char());
    }

    if let Some(token) = Token::keyword(&ident) {
      token
    } else {
      Token::IDENT(ident)
    }
  }

  fn read_number(&mut self) -> Token {
    let mut number = String::new();

    while self.curr.is_ascii_digit() {
      number.push(self.read_char());
    }

    Token::INT(number.parse().unwrap())
  }
}

fn is_letter(c: char) -> bool {
  ('a' <= c && c <= 'z') || ('A' <= c && c <= 'Z') || c == '_'
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::token::Token;

  #[test]
  fn test_generate_tokens() {
    let input = r#"
    let five = 5;
    let ten = 10;
    "#;

    let expected = [
      Token::LET,
      Token::IDENT("five".to_string()),
      Token::ASSIGN,
      Token::INT(5),
      Token::SEMICOLON,
      Token::LET,
      Token::IDENT("ten".to_string()),
      Token::ASSIGN,
      Token::INT(10),
      Token::SEMICOLON,
    ];

    let mut lexer = Lexer::new(input);

    for (i, token) in expected.iter().enumerate() {
      let actual = &lexer.next_token();

      assert_eq!(actual, token, "Failed at token {}", i + 1);
    }
  }
}
