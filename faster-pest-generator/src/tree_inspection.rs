use crate::*;

pub fn list_exprs(expr: &FPestExpr) -> Vec<&FPestExpr> {
    let mut exprs = Vec::new();
    match expr {
        FPestExpr::NegPred(expr) | FPestExpr::Opt(expr) | FPestExpr::Rep(expr, _) => exprs.extend(list_exprs(expr)),
        FPestExpr::Seq(items) | FPestExpr::Choice(items) => items.iter().for_each(|i| exprs.extend(list_exprs(i))),
        FPestExpr::Ident(_) | FPestExpr::Str(_) | FPestExpr::Insens(_) | FPestExpr::CharacterCondition(_) => {},
    }
    exprs.push(expr);
    exprs
}

pub fn contains_idents(expr: &FPestExpr, has_whitespace: bool) -> bool {
    match expr {
        FPestExpr::Ident(ident) if ident != "SOI" && ident != "EOI" && ident != "NEWLINE" => {
            true
        },
        FPestExpr::NegPred(expr) | FPestExpr::Opt(expr) => contains_idents(expr, has_whitespace),
        FPestExpr::Seq(items) => has_whitespace || items.iter().any(|i| contains_idents(i, has_whitespace)),
        FPestExpr::Choice(items) => true, // TODO: items.iter().any(|i| contains_idents(i, has_whitespace)),
        FPestExpr::Rep(expr, _) => has_whitespace || contains_idents(expr, has_whitespace),
        FPestExpr::Str(_) | FPestExpr::Insens(_) | FPestExpr::CharacterCondition(_) => false,
        FPestExpr::Ident(_) => false,
    }
}

pub fn list_choices<'a>(expr: &'a OptimizedExpr, choices: &mut Vec<&'a OptimizedExpr>) {
    if let OptimizedExpr::Choice(first, second) = expr {
        list_choices(first, choices);
        list_choices(second, choices);
    } else {
        choices.push(expr);
    }
}

pub fn list_seq<'a>(expr: &'a OptimizedExpr, seq: &mut Vec<&'a OptimizedExpr>) {
    if let OptimizedExpr::Seq(first, second) = expr {
        list_seq(first, seq);
        list_seq(second, seq);
    } else {
        seq.push(expr);
    }
}
