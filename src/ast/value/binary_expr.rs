use crate::ast::value::{ExprE};

pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Invalid
}
pub struct BinaryExpr {
    pub left: Box<ExprE>,
    pub op: BinOp,
    pub right: Box<ExprE>,
}