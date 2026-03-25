use std::fmt::{Display, Formatter};
use crate::ast::expr::{ExprE};

pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Invalid
}
impl Display for BinOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::Invalid => "invalid",
        })
    }
}
pub struct BinaryExpr {
    pub left: Box<ExprE>,
    pub op: BinOp,
    pub right: Box<ExprE>,
}
