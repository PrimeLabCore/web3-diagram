use syn::{Expr, Ident, Stmt};

pub fn parse_statements(stmts: &[Stmt], functions_called: &mut Vec<Ident>) {
    for st in stmts {
        match st {
            Stmt::Local(l) => {
                if let Some((_, expr)) = &l.init {
                    parse_expr(expr.as_ref(), functions_called);
                }
            }
            Stmt::Item(_) => todo!(),
            Stmt::Expr(_) => todo!(),
            Stmt::Semi(_, _) => todo!(),
        }
    }
}

fn parse_expr(expr: &Expr, functions_called: &mut Vec<Ident>) {
    match expr {
        syn::Expr::Array(_) => todo!(),
        syn::Expr::Assign(_) => todo!(),
        syn::Expr::AssignOp(_) => todo!(),
        syn::Expr::Async(_) => todo!(),
        syn::Expr::Await(_) => todo!(),
        syn::Expr::Binary(_) => todo!(),
        syn::Expr::Block(b) => {
            parse_statements(&b.block.stmts, functions_called);
        }
        syn::Expr::Box(_) => todo!(),
        syn::Expr::Break(_) => todo!(),
        syn::Expr::Call(_) => todo!(),
        syn::Expr::Cast(_) => todo!(),
        syn::Expr::Closure(_) => todo!(),
        syn::Expr::Continue(_) => todo!(),
        syn::Expr::Field(_) => todo!(),
        syn::Expr::ForLoop(_) => todo!(),
        syn::Expr::Group(_) => todo!(),
        syn::Expr::If(_) => todo!(),
        syn::Expr::Index(_) => todo!(),
        syn::Expr::Let(_) => todo!(),
        syn::Expr::Lit(_) => todo!(),
        syn::Expr::Loop(_) => todo!(),
        syn::Expr::Macro(_) => todo!(),
        syn::Expr::Match(_) => todo!(),
        syn::Expr::MethodCall(_) => todo!(),
        syn::Expr::Paren(_) => todo!(),
        syn::Expr::Path(_) => todo!(),
        syn::Expr::Range(_) => todo!(),
        syn::Expr::Reference(_) => todo!(),
        syn::Expr::Repeat(_) => todo!(),
        syn::Expr::Return(_) => todo!(),
        syn::Expr::Struct(_) => todo!(),
        syn::Expr::Try(_) => todo!(),
        syn::Expr::TryBlock(_) => todo!(),
        syn::Expr::Tuple(_) => todo!(),
        syn::Expr::Type(_) => todo!(),
        syn::Expr::Unary(_) => todo!(),
        syn::Expr::Unsafe(_) => todo!(),
        syn::Expr::Verbatim(_) => todo!(),
        syn::Expr::While(_) => todo!(),
        syn::Expr::Yield(_) => todo!(),
        _ => todo!(),
    }
}
