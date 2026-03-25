
use crate::ast::{
    ast_helper::Spanned,
    stmt::decl::SymbolId,
    expr::ExprE
};

pub struct Assignment {
    pub name: Spanned<SymbolId>,
    pub value: ExprE
}