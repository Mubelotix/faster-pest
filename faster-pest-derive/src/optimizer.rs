use pest_meta::optimizer::OptimizedExpr;
use crate::{expr_codegen::CONDITIONS, *};

#[derive(Debug, Clone, PartialEq)]
pub enum FPestExpr {
    Ident(String),
    Str(String),
    Insens(String),
    CharacterCondition(String),
    NegPred(Box<FPestExpr>),
    Seq(Vec<FPestExpr>),
    Choice(Vec<FPestExpr>),
    /// true when empty is accepted
    Rep(Box<FPestExpr>, bool),
    Opt(Box<FPestExpr>),
}

pub fn optimize(expr: &OptimizedExpr) -> FPestExpr {
    match expr {
        OptimizedExpr::Str(value) => {
            if value.len() == 1 {
                FPestExpr::CharacterCondition(format!("c == &b{:?}", value.chars().next().unwrap()))
            } else {
                FPestExpr::Str(value.to_owned())
            }
        },
        OptimizedExpr::Insens(value) => {
            // TODO(optimization): one character optimization
            FPestExpr::Insens(value.to_owned())
        }
        OptimizedExpr::Ident(ident) => {
            if let Some((_, condition)) = CONDITIONS.iter().find(|(n,_)| n==ident) {
                FPestExpr::CharacterCondition(condition.to_string())
            } else {
                FPestExpr::Ident(ident.to_owned())
            }
        },
        OptimizedExpr::NegPred(expr) => {
            FPestExpr::NegPred(Box::new(optimize(expr)))
        }
        OptimizedExpr::Seq(first, second) => {
            if **second == OptimizedExpr::Rep(first.to_owned()) {
                FPestExpr::Rep(Box::new(optimize(first)), false)
            } else {
                let mut seq = Vec::new();
                list_seq(expr, &mut seq);
                FPestExpr::Seq(seq.into_iter().map(optimize).collect())
            }
        }
        OptimizedExpr::Choice(_, _) => {
            let mut choices = Vec::new();
            list_choices(expr, &mut choices);
            
            let mut fp_choices = Vec::new();
            let mut current_condition = String::new();
            for choice in choices {
                let choice = optimize(choice);
                if let FPestExpr::CharacterCondition(c) = choice {
                    if !current_condition.is_empty() {
                        current_condition.push_str(" || ");
                    }
                    current_condition.push_str(&c);
                } else {
                    if !current_condition.is_empty() {
                        fp_choices.push(FPestExpr::CharacterCondition(current_condition));
                        current_condition = String::new();
                    }
                    fp_choices.push(choice);
                }
            }
            if !current_condition.is_empty() {
                fp_choices.push(FPestExpr::CharacterCondition(current_condition));
            }

            if fp_choices.len() == 1 {
                fp_choices.pop().unwrap()
            } else {
                FPestExpr::Choice(fp_choices)
            }
        },
        OptimizedExpr::Opt(expr) => FPestExpr::Opt(Box::new(optimize(expr))),
        OptimizedExpr::Rep(expr) => FPestExpr::Rep(Box::new(optimize(expr)), true),
        OptimizedExpr::PosPred(_) => todo!(),
        OptimizedExpr::Skip(_) => todo!(),
        OptimizedExpr::Push(_) => todo!(),
        OptimizedExpr::RestoreOnErr(_) => todo!(),
        OptimizedExpr::Range(_, _) => todo!(),
        OptimizedExpr::PeekSlice(_, _) => todo!(),
    }
}
