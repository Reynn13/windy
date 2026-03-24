use std::collections::{HashSet};
use crate::lexer::token::{Token, TokenKind};

mod stmt_parser;
mod expr_parser;

use stmt_parser::StmtParser;
use crate::asts::ast_helper::Spanned;
use crate::asts::stmt::decl::SymbolId;
use crate::asts::stmt::StatementE;

struct ParserData<'a> {
    _source: &'a str,
    _tokens: &'a [Token<'a>],
    _idx: usize,
    _symbols: HashSet<&'a str>,
    _id_count: u32
}

impl<'a> ParserData<'a> {
    pub fn new(source: &'a str, tokens: &'a [Token]) -> Self {
        Self {
            _source: source,
            _tokens: tokens,
            _idx: 0,
            _symbols: HashSet::with_capacity(tokens.len() / 3),
            _id_count: 0
        }
    }
}

pub struct Parser<'a> {
    _data: ParserData<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, tokens: &'a [Token]) -> Self {
        Self {
            _data: ParserData::new(source, tokens)
        }
    }

    fn peek(&self) -> Option<&'a Token<'a>> {
        self._data._tokens.get(self._data._idx)
    }
    fn advance(&mut self) -> Option<&'a Token<'a>> {
        self._data._idx += 1;
        self._data._tokens.get(self._data._idx - 1)
    }

    fn consume(&mut self, kind: TokenKind) -> Option<&'a Token<'a>> {
        // TODO: Handle .consume()
        if let Some(tok) = self.peek() {
            if tok.kind == kind {
                Some(tok)
            }
            else {
                None
            }
        }
        else {
            None
        }
    }

    fn get_id(&mut self, token: &'a Token<'a>) -> Spanned<SymbolId> {
        if self._data._symbols.contains(token.lexeme) {
            Spanned {
                node: SymbolId(self._data._id_count),
                loc: token.loc
            }
        } else {
            self._data._symbols.insert(token.lexeme);
            self._data._id_count += 1;
            Spanned {
                node: SymbolId(self._data._id_count - 1),
                loc: token.loc
            }
        }
    }

    pub fn parse_into(&mut self, vec: &mut Vec<StatementE>) {
        while let Some(token) = self.peek() {
            let stmt;
            match (token.kind, token.lexeme) {
                (TokenKind::Identifier, value) => {
                    if value == "decl" {
                        stmt = StmtParser::declaration(self.advance().unwrap().loc, self);
                    }
                    else {  // assigment
                        stmt = StmtParser::assignment(self.advance().unwrap(), self);
                    }
                } 
                _ => {
                    stmt = None;
                    self.advance();
                }
            }
            if let Some(statement) = stmt {
                vec.push(statement);
            }
        }
    }
}