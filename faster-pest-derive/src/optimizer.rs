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
                FPestExpr::CharacterCondition(format!("(c == &{:?})", value.chars().next().unwrap() as u8))
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
                return FPestExpr::Rep(Box::new(optimize(first)), false);
            }

            let mut seq = Vec::new();
            list_seq(expr, &mut seq);
            let mut items = seq.into_iter().map(optimize).collect::<Vec<_>>();

            // Find NegPred(character condition) that are before a character condition
            // and merge them into the character condition
            let mut i = 0;
            while i + 1 < items.len() {
                if let FPestExpr::NegPred(boxed) = &items[i] {
                    if let FPestExpr::CharacterCondition(c) = &**boxed {
                        if let FPestExpr::CharacterCondition(c2) = &items[i + 1] {
                            items[i] = FPestExpr::CharacterCondition(format!("(!{} && {})", c, c2));
                            items.remove(i + 1);
                            continue;
                        } else if let FPestExpr::NegPred(boxed2) = &items[i + 1] {
                            if let FPestExpr::CharacterCondition(c2) = &**boxed2 {
                                items[i] = FPestExpr::NegPred(Box::new(FPestExpr::CharacterCondition(format!("({} || {})", c, c2))));
                                items.remove(i + 1);
                                continue;
                            }
                        }
                    }
                }
                i += 1;
            }

            if items.len() == 1 {
                items.pop().unwrap()
            } else {
                FPestExpr::Seq(items)
            }
        }
        OptimizedExpr::Choice(_, _) => {
            let mut choices = Vec::new();
            list_choices(expr, &mut choices);
            
            // Group character conditions that are next to each other
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
