use std::collections::{HashMap};
use crate::lexer::token::{Token, TokenKind};

mod stmt_parser;
mod expr_parser;

use stmt_parser::StmtParser;
use crate::ast::ast_helper::Spanned;
use crate::ast::stmt::decl::SymbolId;
use crate::ast::stmt::StatementE;

struct ParserInterner<'a> {
    _map: HashMap<&'a str, SymbolId>,
    _vec: Vec<&'a str>,
}

struct ParserData<'a> {
    _source: &'a str,
    _tokens: &'a [Token<'a>],
    _idx: usize,
    _interner: ParserInterner<'a>,
}

impl<'a> ParserData<'a> {
    pub fn new(source: &'a str, tokens: &'a [Token]) -> Self {
        Self {
            _source: source,
            _tokens: tokens,
            _idx: 0,
            _interner: ParserInterner {
                _map: HashMap::with_capacity(tokens.len() / 3),
                _vec: Vec::with_capacity(tokens.len() / 3),
            }
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
        if let Some(node) = self._data._interner._map.get(token.lexeme) {
            Spanned {
                node: *node,
                loc: token.loc
            }
        } else {
            let id = SymbolId(self._data._interner._vec.len());
            self._data._interner._map.insert(token.lexeme, id);
            self._data._interner._vec.push(token.lexeme);
            Spanned {
                node: id,
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