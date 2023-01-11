use pest_meta::optimizer::OptimizedExpr;

pub fn list_exprs(expr: &OptimizedExpr, ignore_self: bool) -> Vec<&OptimizedExpr> {
    let mut exprs = Vec::new();
    match expr {
        OptimizedExpr::PosPred(expr) | OptimizedExpr::NegPred(expr) | OptimizedExpr::Opt(expr) | OptimizedExpr::Rep(expr) | OptimizedExpr::Push(expr) | OptimizedExpr::RestoreOnErr(expr) => exprs.extend(list_exprs(expr, false)),
        OptimizedExpr::Seq(first, second) => {
            exprs.extend(list_exprs(first, matches!(**first, OptimizedExpr::Seq(_, _))));
            exprs.extend(list_exprs(second, matches!(**second, OptimizedExpr::Seq(_, _))));
        }
        OptimizedExpr::Choice(first, second) => {
            exprs.extend(list_exprs(first, matches!(**first, OptimizedExpr::Choice(_, _))));
            exprs.extend(list_exprs(second, matches!(**second, OptimizedExpr::Choice(_, _))));
        }
        _ => ()
    }
    if !ignore_self {
        exprs.push(expr);
    }
    exprs
}

pub fn contains_idents(expr: &OptimizedExpr, has_whitespace: bool) -> bool {
    match expr {
        OptimizedExpr::Ident(ident) if ident != "SOI" && ident != "EOI" && ident != "NEWLINE" && !crate::expr_codegen::CONDITIONS.iter().any(|(n,_)| n==ident) => {
            true
        },
        OptimizedExpr::PosPred(expr) | OptimizedExpr::NegPred(expr) | OptimizedExpr::Opt(expr) | OptimizedExpr::Push(expr) | OptimizedExpr::RestoreOnErr(expr) => contains_idents(expr, has_whitespace),
        OptimizedExpr::Seq(first, second) => has_whitespace || contains_idents(first, has_whitespace) || contains_idents(second, has_whitespace),
        OptimizedExpr::Choice(first, second) => contains_idents(first, has_whitespace) || contains_idents(second, has_whitespace),
        OptimizedExpr::Rep(expr) => has_whitespace || contains_idents(expr, has_whitespace),
        _ => false
    }
}

pub fn list_choices<'a, 'b>(expr: &'a OptimizedExpr, choices: &'b mut Vec<&'a OptimizedExpr>) {
    if let OptimizedExpr::Choice(first, second) = expr {
        list_choices(first, choices);
        list_choices(second, choices);
    } else {
        choices.push(expr);
    }
}

pub fn list_seq<'a, 'b>(expr: &'a OptimizedExpr, seq: &'b mut Vec<&'a OptimizedExpr>) {
    if let OptimizedExpr::Seq(first, second) = expr {
        list_seq(first, seq);
        list_seq(second, seq);
    } else {
        seq.push(expr);
    }
}
