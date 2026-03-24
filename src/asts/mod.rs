
pub mod stmt;
pub mod ast_helper;
pub mod value;
pub mod types;

use stmt::StatementE;
use value::ExprE;

pub enum AstE {
    Statement(StatementE),
    Expr(ExprE)
}