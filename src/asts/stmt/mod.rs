
pub mod decl;
pub mod assign;

use decl::Declaration;
use assign::Assignment;
use crate::asts::ast_helper::Spanned;

pub enum StatementE {
    DeclarationE(Spanned<Declaration>),
    AssigmentE(Spanned<Assignment>)
}