
use crate::ast::{
    ast_helper::Spanned,
    stmt::decl::SymbolId,
    value::ExprE
};

pub struct Assignment {
    pub name: Spanned<SymbolId>,
    pub value: Option<ExprE>
}