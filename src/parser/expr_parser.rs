use crate::ast::ast_helper::Spanned;
use crate::ast::expr::{ExprE};
use crate::ast::expr::binary_expr::{BinOp, BinaryExpr};
use crate::ast::expr::literal::Literal;
use crate::lexer::token::TokenKind;
use crate::parser::Parser;

pub struct ExprParser;

impl ExprParser {
    // Return can be None if met eof
    fn prefix (dispatcher: &mut Parser) -> Option<ExprE> {
        while let Some(tok) = dispatcher.advance() {
            match tok.kind {
                TokenKind::Identifier => return Some(ExprE::IdentifierE(
                    dispatcher.get_id(tok)
                )),
                TokenKind::Integer => return Some(ExprE::LiteralE(
                    Spanned {
                        node: Literal::Number(tok.lexeme.parse().unwrap()),
                        loc: tok.loc
                    }
                )),
                _ => {}
            }
        }
        None
    }
    pub fn parse_into(min_precedence: u8, value: &mut Option<ExprE>, dispatcher: &mut Parser) {
        let left = Self::prefix(dispatcher);
        if left.is_none() {
            *value = None;
            return;
        }
        let mut left = left.unwrap();
        
        while let Some(op) = dispatcher.peek() {
            if op.kind == TokenKind::Semicolon {
                break;
            }
            let precedence: u8 = match op.kind {
                TokenKind::Add => 1,
                TokenKind::Sub => 1,
                TokenKind::Mul => 2,
                TokenKind::Div => 2,
                _ => 0 // TODO: Handle unknown operator
            };
            if precedence < min_precedence {
                break;
            }
            dispatcher.advance();
            let op = match op.kind {
                TokenKind::Add => BinOp::Add,
                TokenKind::Sub => BinOp::Sub,
                TokenKind::Mul => BinOp::Mul,
                TokenKind::Div => BinOp::Div,
                _ => unreachable!()
            };
            
            let mut right = None;
            Self::parse_into(precedence + 1, &mut right, dispatcher);
            if right.is_none() {
                break;
            } 
            let right = right.unwrap();
            
            left = ExprE::BinaryExprE(
                Spanned {
                    loc: left.get_loc() + right.get_loc(),
                    node: BinaryExpr {
                        left: Box::new(left),
                        op,
                        right: Box::new(right),
                    }
                }
            )
        }
        *value = Some(left);
    }
}