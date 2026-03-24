
use crate::asts::{
    ast_helper::Spanned,
    types::TypeExpr,
    value::ExprE
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SymbolId(pub u32);

pub struct Declaration {
    pub name: Spanned<SymbolId>,
    pub ty: Option<Spanned<TypeExpr>>,
    pub value: Option<ExprE>
}