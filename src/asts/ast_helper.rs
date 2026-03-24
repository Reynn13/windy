use crate::lexer::spanloc::SpanLoc;

pub struct Spanned<T> {
    pub node: T,
    pub loc: SpanLoc
}