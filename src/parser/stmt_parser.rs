use crate::ast::ast_helper::Spanned;
use crate::ast::stmt::assign::Assignment;
use crate::ast::stmt::decl::Declaration;
use crate::ast::stmt::StatementE;
use crate::lexer::spanloc::SpanLoc;
use crate::lexer::token::{Token, TokenKind};
use crate::parser::Parser;
use crate::parser::expr_parser::ExprParser;

pub struct StmtParser;

impl StmtParser {

    // [Tokens] | Current token -> The Identifier
    pub fn declaration(start_loc: SpanLoc, dispatcher: &mut Parser) -> Option<StatementE> {
        let ident = dispatcher.consume(TokenKind::Identifier);
        if ident.is_none() {
            return None;
        }
        if dispatcher.consume(TokenKind::Equal).is_none() {
            return None;
        }
        let mut value = None;
        ExprParser::parse_into(0, &mut value, dispatcher);
        
        let end = dispatcher.consume(TokenKind::Semicolon);
        if end.is_none() {
            return None;
        }
        let end = end.unwrap().loc;
        
        Some(StatementE::DeclarationE(
            Spanned {
                node: Declaration {
                    name: dispatcher.get_id(ident.unwrap()),
                    ty: None, // TODO: Support statically typed variable
                    value,
                },
                loc: start_loc + end
            }
        ))
    }
    
    pub fn assignment<'a>(ident: &'a Token<'_>, dispatcher: &mut Parser<'a>) -> Option<StatementE> {
        if dispatcher.consume(TokenKind::Equal).is_none() {
            return None;
        }
        let mut value = None;
        ExprParser::parse_into(0, &mut value, dispatcher);
        if value.is_none() {
            return None;
        }
        let value = value.unwrap();

        let end = dispatcher.consume(TokenKind::Semicolon);
        if end.is_none() {
            return None;
        }
        let end = end.unwrap().loc;

        Some(StatementE::AssigmentE(
            Spanned {
                node: Assignment {
                    name: dispatcher.get_id(ident),
                    value,
                },
                loc: ident.loc + end
            }
        ))
    }
}