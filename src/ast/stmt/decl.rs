
use crate::ast::{
    ast_helper::Spanned,
    types::TypeExpr,
    expr::ExprE
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SymbolId(pub usize);

pub struct Declaration {
    pub name: Spanned<SymbolId>,
    pub ty: Option<Spanned<TypeExpr>>,
    pub value: Option<ExprE>
}