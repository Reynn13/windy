use std::{
    fmt::{Display, Formatter},
    cmp::PartialEq
};
use crate::lexer::spanloc::SpanLoc;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum TokenKind {
    // Symbols
    Identifier,

    // Values
    Integer,
    Float,

    // Operators
    Colon,
    Equal,
    Semicolon,
    
    // Arithmetic ops
    Add,
    Sub,
    Mul,
    Div,

    // Others
    Unknown
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            TokenKind::Identifier => "IDENTIFIER",

            TokenKind::Integer => "INTEGER",
            TokenKind::Float => "FLOAT",

            TokenKind::Colon => "COLON",
            TokenKind::Equal => "EQUAL",
            TokenKind::Semicolon => "SEMICOLON",
            
            TokenKind::Add => "ADD",
            TokenKind::Sub => "SUB",
            TokenKind::Mul => "MUL",
            TokenKind::Div => "DIV",

            TokenKind::Unknown => "UNKNOWN"
        })
    }
}


#[derive(Clone)]
pub struct Token<'a> {
    //lexeme: &'a str,
    pub kind: TokenKind,
    pub lexeme: &'a str,
    pub loc: SpanLoc
}

impl<'a> Token<'a> {
    pub fn new(lexeme: &'a str, kind: TokenKind, loc: SpanLoc) -> Self {
        Self {
            kind,
            lexeme,
            loc
        }
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //write!(f, "[{}({}) | {}]", self.kind, self.lexeme, self.loc)
        write!(f, "[{} | {}]", self.kind, self.loc)
    }
}

impl PartialEq<TokenKind> for Token<'_> {
    fn eq(&self, other: &TokenKind) -> bool {
        self.kind == *other
    }
}