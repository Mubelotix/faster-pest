use pest_meta::optimizer::OptimizedExpr;
use crate::*;

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

pub fn optimize<G: Generator>(expr: &OptimizedExpr) -> FPestExpr {
    match expr {
        OptimizedExpr::Str(value) => {
            if value.len() == 1 {
                FPestExpr::CharacterCondition(G::character(value.as_bytes()[0]))
            } else {
                FPestExpr::Str(value.to_owned())
            }
        },
        OptimizedExpr::Insens(value) => {
            // TODO(optimization): one character optimization
            FPestExpr::Insens(value.to_owned())
        }
        OptimizedExpr::Ident(ident) => {
            if let Some(condition) = G::character_ident(ident) {
                FPestExpr::CharacterCondition(condition.to_string())
            } else {
                FPestExpr::Ident(ident.to_owned())
            }
        },
        OptimizedExpr::NegPred(expr) => {
            FPestExpr::NegPred(Box::new(optimize::<G>(expr)))
        }
        OptimizedExpr::Seq(first, second) => {
            if **second == OptimizedExpr::Rep(first.to_owned()) {
                return FPestExpr::Rep(Box::new(optimize::<G>(first)), false);
            }

            let mut seq = Vec::new();
            list_seq(expr, &mut seq);
            let mut items = seq.into_iter().map(optimize::<G>).collect::<Vec<_>>();

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
                let choice = optimize::<G>(choice);
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
        OptimizedExpr::Opt(expr) => FPestExpr::Opt(Box::new(optimize::<G>(expr))),
        OptimizedExpr::Rep(expr) => FPestExpr::Rep(Box::new(optimize::<G>(expr)), true),
        OptimizedExpr::Range(a, b) => {
            if a.len() == 1 && b.len() == 1 {
                let a = a.chars().next().unwrap() as u8;
                let b = b.chars().next().unwrap() as u8;
                FPestExpr::CharacterCondition(G::character_range(a, b))
            } else {
                todo!()
            }
        }
        OptimizedExpr::PosPred(_) => todo!(),
        OptimizedExpr::Skip(_) => todo!(),
        OptimizedExpr::Push(_) => todo!(),
        OptimizedExpr::RestoreOnErr(_) => todo!(),
        OptimizedExpr::PeekSlice(_, _) => todo!(),
    }
}

pub fn optimize_second_stage(expr: &mut FPestExpr, character_set_rules: &HashMap<&str, String>) {
    match expr {
        FPestExpr::Ident(ident) => if let Some(condition) = character_set_rules.get(ident.as_str()) {
            *expr = FPestExpr::CharacterCondition(condition.to_string());
        },
        FPestExpr::Str(_) => (),
        FPestExpr::Insens(_) => (),
        FPestExpr::CharacterCondition(_) => (),
        FPestExpr::NegPred(expr) => optimize_second_stage(expr, character_set_rules),
        FPestExpr::Seq(items) => {
            for item in items.iter_mut() {
                optimize_second_stage(item, character_set_rules);
            }

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
                *expr = items.pop().unwrap();
            }
        },
        FPestExpr::Choice(items) => {
            // Group character conditions that are next to each other
            let mut fp_choices = Vec::new();
            let mut current_condition = String::new();
            for item in items.iter_mut() {
                optimize_second_stage(item, character_set_rules);
                if let FPestExpr::CharacterCondition(c) = item {
                    if !current_condition.is_empty() {
                        current_condition.push_str(" || ");
                    }
                    current_condition.push_str(c);
                } else {
                    if !current_condition.is_empty() {
                        fp_choices.push(FPestExpr::CharacterCondition(current_condition));
                        current_condition = String::new();
                    }
                    fp_choices.push(item.to_owned());
                }
            }
            if !current_condition.is_empty() {
                fp_choices.push(FPestExpr::CharacterCondition(current_condition));
            }

            if fp_choices.len() == 1 {
                *expr = fp_choices.pop().unwrap()
            } else {
                *expr = FPestExpr::Choice(fp_choices);
            }
        },
        FPestExpr::Rep(expr, _) => optimize_second_stage(expr, character_set_rules),
        FPestExpr::Opt(expr) => optimize_second_stage(expr, character_set_rules),
    }
}
