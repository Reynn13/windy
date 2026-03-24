use crate::ast::ast_helper::Spanned;


pub enum Literal {
    Number(i64),
    Float(f64),
    String(String),
}