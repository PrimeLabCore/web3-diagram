use syn::{Expr, Ident, Stmt};

// Extracting information from statements of block
pub fn parse_statements(stmts: &[Stmt], functions_called: &mut Vec<Ident>) {
    for st in stmts {
        match st {
            Stmt::Local(l) => {
                if let Some((_, expr)) = &l.init {
                    parse_expr(expr.as_ref(), functions_called);
                }
            }
            Stmt::Item(it) => parse_item(it, functions_called),
            Stmt::Expr(expr) => parse_expr(expr, functions_called),
            Stmt::Semi(_, _) => todo!(),
        }
    }
}

fn parse_item(it: &syn::Item, functions_called: &mut Vec<Ident>) {
    match it {
        syn::Item::Const(c) => parse_expr(c.expr.as_ref(), functions_called),
        syn::Item::Static(s) => parse_expr(s.expr.as_ref(), functions_called),
        syn::Item::Fn(_) => todo!(), // TODO: discuss if we show it in diagram
        // TODO: probably skip those:
        syn::Item::ForeignMod(_) => todo!(),
        syn::Item::Mod(_) => todo!(),

        syn::Item::Enum(_)
        | syn::Item::ExternCrate(_)
        | syn::Item::Impl(_)
        | syn::Item::Macro(_)
        | syn::Item::Macro2(_)
        | syn::Item::Struct(_)
        | syn::Item::Trait(_)
        | syn::Item::TraitAlias(_)
        | syn::Item::Type(_)
        | syn::Item::Union(_)
        | syn::Item::Use(_)
        | syn::Item::Verbatim(_) => {}
        _ => {}
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

        // TODO: Kinda hard, can get even more complex,
        // if we try to check use(in cases of functions with same name from different modules/impls/etc)
        syn::Expr::Call(c) => todo!(),

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
