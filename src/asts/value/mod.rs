use crate::asts::ast_helper::Spanned;
use crate::asts::stmt::decl::SymbolId;


pub mod literal;
use literal::Literal;

pub mod binary_expr;
use binary_expr::BinaryExpr;
use crate::asts::value::ExprE::{BinaryExprE, IdentifierE, LiteralE};
use crate::lexer::spanloc::SpanLoc;

pub enum ExprE {
    LiteralE(Spanned<Literal>),
    BinaryExprE(Spanned<BinaryExpr>),
    IdentifierE(Spanned<SymbolId>),
}
impl ExprE {
    pub fn get_loc(&self) -> SpanLoc {
        match self {
            LiteralE(l) => l.loc,
            BinaryExprE(b) => b.loc,
            IdentifierE(i) => i.loc,
        }
    }
}