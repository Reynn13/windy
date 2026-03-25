use windy::ast::displayer::AstDebugDisplayer;
use windy::lexer::Lexer;
use windy::parser::Parser;

fn main() {
    let src = "decl a = 12 + 2;";
    let mut lexer = Lexer::new(src);
    let mut tokens = Vec::new();
    lexer.lex_into(&mut tokens);

    let mut parser = Parser::new(src, &*tokens);
    let mut asts = Vec::new();
    parser.parse_into(&mut asts);

    AstDebugDisplayer::print(&asts, src);

}