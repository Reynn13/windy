use crate::ast::ast_helper::Spanned;
use crate::ast::expr::ExprE;
use crate::ast::expr::literal::Literal;
use crate::ast::stmt::assign::Assignment;
use crate::ast::stmt::decl::{Declaration};
use crate::ast::stmt::StatementE;
use crate::lexer::spanloc::SpanLoc;

pub struct AstDebugDisplayer;

impl AstDebugDisplayer {
    fn src_from_loc<'a>(src: &'a str, loc: &SpanLoc) -> &'a str {
        &src[loc.start_idx as usize  .. loc.end_idx as usize]
    }
    fn print_ident(count: usize) {
        let colors = [
            "\x1b[31m",
            "\x1b[32m",
            "\x1b[33m",
            "\x1b[34m",
            "\x1b[35m",
            "\x1b[36m",
        ];

        let mut last_idx = 0;
        for i in 0..count {
            let color = colors[i % colors.len()];
            print!("{}— \x1b[0m", color);
            last_idx = i;
        }

        print!("{}→ \x1b[0m", colors[(last_idx + 1) % colors.len()]);
    }
    fn print_expr(indent: usize, value: &ExprE, src: &str) {
        Self::print_ident(indent);
        let loc: SpanLoc;
        match value {
            ExprE::LiteralE(lit) => {
                match &lit.node {
                    Literal::Number(n) => println!("Literal(I64, {})", n),
                    Literal::Float(f) => println!("Literal(F64, {})", f),
                    Literal::String(s) => println!("Literal(String, {})", s),
                }
                loc = lit.loc;
            },
            ExprE::BinaryExprE(bin) => {
                println!("BinaryExpr({}):", bin.node.op);
                Self::print_ident(indent +1);
                println!("Left:");
                Self::print_expr(indent +2, &*bin.node.left, src);

                Self::print_ident(indent +1);
                println!("Right:");
                Self::print_expr(indent +2, &*bin.node.right, src);

                loc = bin.loc;
            },
            ExprE::IdentifierE(ident) => {
                println!("Identifier({}):", Self::src_from_loc(src, &ident.loc));
                loc = ident.loc;
            }
        }
        Self::print_ident(indent);
        println!("Loc:");
        Self::print_ident(indent +1);
        println!("{}", loc);
    }
    fn print_decl(indent: usize, decl: &Spanned<Declaration>, src: &str) {
        Self::print_ident(indent);
        println!("Declaration({}):", Self::src_from_loc(src, &decl.node.name.loc));
        if let Some(ty) = &decl.node.ty {
            Self::print_ident(indent +1);
            println!("Type: {}", Self::src_from_loc(src, &ty.loc))
        }
        if let Some(val) = &decl.node.value {
            Self::print_ident(indent +1);
            println!("Value:");
            Self::print_expr(indent +2, val, src)
        }
        Self::print_ident(indent +1);
        println!("Loc:");
        Self::print_ident(indent +2);
        println!("{}", decl.loc);
    }
    fn print_ass(indent: usize, ass: &Spanned<Assignment>, src: &str) {
        Self::print_ident(indent);
        println!("Declaration({}):", Self::src_from_loc(src, &ass.node.name.loc));

        Self::print_ident(indent +1);
        println!("Value:");
        Self::print_expr(indent +2, &ass.node.value, src);

        Self::print_ident(indent +1);
        println!("Loc:");
        Self::print_ident(indent +2);
        println!("{}", ass.loc);
    }
    pub fn print(vec: &Vec<StatementE>, src: &str) {
        for statement in vec {
            match statement {
                StatementE::DeclarationE(decl) => Self::print_decl(0, decl, src),
                StatementE::AssigmentE(ass) => Self::print_ass(0, ass, src)
            }
        }
    }
}