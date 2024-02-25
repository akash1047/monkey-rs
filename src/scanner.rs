use crate::token::Token;

pub struct Scanner {
    src: String, // original source chracter stream

    ch: u8,           // currently processing character
    offset: usize,    // position of the ch
    rd_offset: usize, // position of the next character
}

impl Scanner {
    pub fn new(src: String) -> Self {
        let mut s = Self {
            src,

            ch: b' ',
            offset: 0,
            rd_offset: 0,
        };
        s.next();
        s
    }

    fn next(&mut self) {
        if self.rd_offset < self.src.len() {
            self.offset = self.rd_offset;
            self.ch = self.src.as_bytes()[self.rd_offset];
            self.rd_offset += 1;
        } else {
            self.offset = self.src.len();
            self.ch = 0;
        }
    }

    fn peek(&self) -> u8 {
        *self.src.as_bytes().get(self.rd_offset).unwrap_or(&0)
    }

    #[inline]
    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.next();
        }
    }

    pub fn switch(&mut self, def: Token, ch: u8, alt: Token) -> Token {
        if self.peek() == ch {
            self.next();
            alt
        } else {
            def
        }
    }

    fn switch2(&mut self, def: Token, ch1: u8, tok1: Token, ch2: u8, tok2: Token) -> Token {
        match self.peek() {
            ch1 => {
                self.next();
                tok1
            }

            ch2 => {
                self.next();
                tok2
            }
            _ => def,
        }
    }

    pub fn scan(&mut self) -> (Token, &str) {
        self.skip_whitespace();

        let offset = self.offset;

        if is_letter(self.ch) && !is_digit(self.ch) {
            while is_letter(self.ch) {
                self.next();
            }

            let lit = &self.src[offset..self.offset];

            let tok = match lit {
                "fn" => Token::FN,
                "let" => Token::LET,
                "return" => Token::RETURN,
                "if" => Token::IF,
                "else" => Token::ELSE,
                "true" => Token::TRUE,
                "false" => Token::FALSE,
                _ => Token::IDENT,
            };

            return (tok, lit);
        }

        if is_digit(self.ch) {
            while is_digit(self.ch) {
                self.next();
            }

            let lit = &self.src[offset..self.offset];

            return (Token::INTEGER, lit);
        }

        let tok = match self.ch {
            b'=' => self.switch(Token::ASSIGN, b'=', Token::EQ),

            b';' => Token::SEMICOLON,
            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b',' => Token::COMMA,

            b'+' => Token::PLUS,

            b'-' => Token::MINUS,
            b'*' => Token::ASTERISK,
            b'/' => Token::SLASH,

            b'!' => self.switch(Token::BANG, b'=', Token::NEQ),

            b'<' => self.switch(Token::LT, b'=', Token::LEQ),
            b'>' => self.switch(Token::GT, b'=', Token::GEQ),

            b'{' => Token::LBRACE,
            b'}' => Token::RBRACE,
            0 => return (Token::EOF, ""),
            _ => Token::ILLEGAL,
        };

        self.next();

        (tok, &self.src[offset..self.offset])
    }
}

fn is_digit(c: u8) -> bool {
    c >= b'0' && c <= b'9'
}

fn is_letter(c: u8) -> bool {
    c >= b'a' && c <= b'z' || c >= b'A' && c <= b'Z' || c >= b'0' && c <= b'9' || c == b'_'
}

#[cfg(test)]
mod tests {

    use super::*;
    use Token::*;

    #[test]
    fn test_scan() {
        let src = "let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);

!-/*5;
5 < 10> 5;

if (5 < 10) {
  return true;
} else {
  return false;
}

10 == 10;
10 != 9;
9 <= 9;
10 >= 10;
";

        let tests = [
            // let five = 5;
            (LET, "let"),
            (IDENT, "five"),
            (ASSIGN, "="),
            (INTEGER, "5"),
            (SEMICOLON, ";"),
            // let ten = 10;
            (LET, "let"),
            (IDENT, "ten"),
            (ASSIGN, "="),
            (INTEGER, "10"),
            (SEMICOLON, ";"),
            // let add = fn(x, y) {
            //   x + y;
            // };
            (LET, "let"),
            (IDENT, "add"),
            (ASSIGN, "="),
            (FN, "fn"),
            (LPAREN, "("),
            (IDENT, "x"),
            (COMMA, ","),
            (IDENT, "y"),
            (RPAREN, ")"),
            (LBRACE, "{"),
            (IDENT, "x"),
            (PLUS, "+"),
            (IDENT, "y"),
            (SEMICOLON, ";"),
            (RBRACE, "}"),
            (SEMICOLON, ";"),
            // let result = add(five, ten);
            (LET, "let"),
            (IDENT, "result"),
            (ASSIGN, "="),
            (IDENT, "add"),
            (LPAREN, "("),
            (IDENT, "five"),
            (COMMA, ","),
            (IDENT, "ten"),
            (RPAREN, ")"),
            (SEMICOLON, ";"),
            // !-/*5;
            (BANG, "!"),
            (MINUS, "-"),
            (SLASH, "/"),
            (ASTERISK, "*"),
            (INTEGER, "5"),
            (SEMICOLON, ";"),
            // 5 < 10> 5;
            (INTEGER, "5"),
            (LT, "<"),
            (INTEGER, "10"),
            (GT, ">"),
            (INTEGER, "5"),
            (SEMICOLON, ";"),
            // if (5 < 10) {
            //   return true;
            // } else {
            //   return false;
            // }
            (IF, "if"),
            (LPAREN, "("),
            (INTEGER, "5"),
            (LT, "<"),
            (INTEGER, "10"),
            (RPAREN, ")"),
            (LBRACE, "{"),
            (RETURN, "return"),
            (TRUE, "true"),
            (SEMICOLON, ";"),
            (RBRACE, "}"),
            (ELSE, "else"),
            (LBRACE, "{"),
            (RETURN, "return"),
            (FALSE, "false"),
            (SEMICOLON, ";"),
            (RBRACE, "}"),
            // 10 == 10;
            (INTEGER, "10"),
            (EQ, "=="),
            (INTEGER, "10"),
            (SEMICOLON, ";"),
            // 10 != 9;
            (INTEGER, "10"),
            (NEQ, "!="),
            (INTEGER, "9"),
            (SEMICOLON, ";"),
            // 9 <= 9;
            (INTEGER, "9"),
            (LEQ, "<="),
            (INTEGER, "9"),
            (SEMICOLON, ";"),
            // 10 >= 10;
            (INTEGER, "10"),
            (GEQ, ">="),
            (INTEGER, "10"),
            (SEMICOLON, ";"),
            // end of file
            (EOF, ""),
        ];

        let mut scanner = Scanner::new(src.to_string());

        for (i, t) in tests.iter().enumerate() {
            let tok = scanner.scan();
            assert_eq!(*t, tok, "[{}/{}] test failed.", i + 1, tests.len());
        }
    }
}
