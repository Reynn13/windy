
use crate::lexer::{
    spanloc::SpanLoc, 
    token::{Token, TokenKind}
};

pub mod token;
pub mod spanloc;

struct LexerData<'a> {
    _source: &'a str,
    _idx: usize,
    _col: usize,
    _line: usize,
}

impl<'a> LexerData<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            _source: source,
            _idx: 0,
            _col: 1,
            _line: 1
        }
    }

    pub fn peek(&self) -> Option<u8> {
        self._source.as_bytes().get(self._idx).copied()
    }
    pub fn consume(&mut self) -> Option<u8> {
        let ch = self.peek()?;
        self._idx += 1;

        if ch == b'\n' {
            self._line += 1;
            self._col = 1;
        } else {
            self._col += 1;
        }

        Some(ch)
    }


    pub fn skip(&mut self) {
        self._idx += 1;
        self._col += 1;
    }

}

pub struct Lexer<'a> {
    _data: LexerData<'a>
}


impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
          _data: LexerData::new(source)
        }
    }
    fn lex_identifier(&mut self) -> Token<'a> {
        let (si, sc) = (self._data._idx, self._data._col);
        while let Some(ch) = self._data.peek() {
            if ch.is_ascii_alphanumeric() || ch == b'_' {
                self._data.skip();
            } else {
                break;
            }
        }
        Token::new(
            &self._data._source[si..self._data._idx],
            TokenKind::Identifier,
            SpanLoc::new(
                si, self._data._idx,
                sc, self._data._col,
                self._data._line, self._data._line
            )
        )
    }
    fn lex_number(&mut self) -> Token<'a> {
        let (si, sc) = (self._data._idx, self._data._col);
        while let Some(ch) = self._data.peek() {
            if ch >= b'0' && ch <= b'9' {
                self._data.skip();
            }
            else {
                break
            }
        }
        Token::new(
            &self._data._source[si..self._data._idx],
            TokenKind::Integer,
            SpanLoc::new(
                si, self._data._idx,
                sc, self._data._col,
                self._data._line, self._data._line
            )
        )
    }

    fn next_token(&mut self, ch: u8) -> Token<'a> {
        if ch.is_ascii_digit() {
            return self.lex_number();
        }
        if ch.is_ascii_alphabetic() || ch == b'_' {
            return self.lex_identifier();
        }
        self._data.skip();
        let kind: TokenKind = match ch {
            b':' => TokenKind::Colon,
            b'=' => TokenKind::Equal,
            b';' => TokenKind::Semicolon,
            
            b'+' => TokenKind::Add,
            b'-' => TokenKind::Sub,
            b'*' => TokenKind::Mul,
            b'/' => TokenKind::Div,

            _ => TokenKind::Unknown
        };
        Token::new(
            &self._data._source[(self._data._idx - 1)..self._data._idx],
            kind,
            SpanLoc::new(
                self._data._idx - 1, self._data._idx,
                self._data._col - 1, self._data._col,
                self._data._line, self._data._line
            )
        )
    }

    fn inner_lex(&mut self, vec: &mut Vec<Token<'a>>) {
        while let Some(ch) = self._data.peek() {
            if ch.is_ascii_whitespace() {
                self._data.consume();
            } else {
                vec.push(self.next_token(ch));
            }
        }
    }
    pub fn lex_into(&mut self, vec: &mut Vec<Token<'a>>){
        self.inner_lex(vec);
    }

    pub fn lex_into_and_reset(&mut self, vec: &mut Vec<Token<'a>>) {
        self._data._idx = 0;
        self._data._col = 1;
        self._data._line = 1;
        self.inner_lex(vec);
    }
}