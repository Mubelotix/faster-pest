use crate::{*, optimizer::FPestExpr};

fn to_pest(expr: &FPestExpr) -> String {
    match expr {
        FPestExpr::Str(s) => format!("{s:?}"),
        FPestExpr::CharacterCondition(c) => format!("({c})"),
        FPestExpr::Insens(s) => format!("^{s:?}"),
        FPestExpr::Ident(i) => i.to_owned(),
        FPestExpr::NegPred(e) => format!("!{}", to_pest(e)),
        FPestExpr::Seq(exprs) => format!("({})", exprs.iter().map(to_pest).collect::<Vec<_>>().join(" ~ ")),
        FPestExpr::Choice(exprs) => format!("({})", exprs.iter().map(to_pest).collect::<Vec<_>>().join(" | ")),
        FPestExpr::Opt(e) => format!("{}?", to_pest(e)),
        FPestExpr::Rep(e, true) => format!("{}*", to_pest(e)),
        FPestExpr::Rep(e, false) => format!("{}+", to_pest(e)),
    }
}

pub fn code<G: Generator>(expr: &FPestExpr, ids: &mut IdRegistry, has_whitespace: bool) -> String {
    let id = ids.id(expr);
    let formatted_idents = match contains_idents(expr, has_whitespace) {
        true => "idents: &'b mut Vec<(Ident<'i>, usize)>",
        false => "",
    };
    let hr_expr = to_pest(expr);
    match expr {
        FPestExpr::Ident(ident) => {
            let mut code = G::ident(ident);
            code = code.replace("expr_id", &id);
            code
        },
        FPestExpr::CharacterCondition(condition) => {
            let mut code = G::pattern_expr_character().to_owned();
            code = code.replace("expr_id", &id);
            code = code.replace("expr_pest", &hr_expr);
            code = code.replace("formatted_idents", formatted_idents);
            code = code.replace("character_condition", condition);
            code
        }
        FPestExpr::Choice(items) => {
            let mut code = G::pattern_expr_choice().to_owned();
            code = code.replace("expr_id", &id);
            code = code.replace("expr_pest", &hr_expr);
            code = code.replace("formatted_idents", formatted_idents);
            code = multi_replace(code, vec![
                ("choice_item_id", items.iter().map(|item| ids.id(item)).collect::<Vec<_>>()),
                ("choice_idents", items.iter().map(|item| {
                    match contains_idents(item, has_whitespace) {
                        true => "idents",
                        false => "",
                    }.to_string()
                }).collect::<Vec<_>>()),
            ]);
            code
        }
        FPestExpr::Str(value) => {
            let mut code = G::pattern_expr_str().to_owned();
            code = code.replace("expr_id", &id);
            code = code.replace("expr_pest", &hr_expr);
            code = code.replace("expr_str", format!("{value:?}").as_str());
            code
        }
        FPestExpr::Seq(items) => {
            let mut code = G::pattern_expr_seq().to_owned();
            code = code.replace("expr_id", &id);
            code = code.replace("expr_pest", &hr_expr);
            code = code.replace("formatted_idents", formatted_idents);
            code = multi_replace(code, vec![
                ("seq_item_id", items.iter().map(|item| ids.id(item)).collect::<Vec<_>>()),
                ("seq_idents", items.iter().map(|item| {
                    match contains_idents(item, has_whitespace) {
                        true => "idents",
                        false => "",
                    }.to_string()
                }).collect::<Vec<_>>()),
                ("seq_n", (0..items.len()).map(|i| i.to_string()).collect::<Vec<_>>()),
            ]);
            if has_whitespace {
                code = code.replace("//WSP", " ");
            }
            code
        }
        FPestExpr::Rep(expr, empty_accepted) => {
            if let FPestExpr::CharacterCondition(condition) = &**expr {
                let mut code = G::pattern_expr_rep_character().to_owned();
                code = code.replace("expr_id", &id);
                code = code.replace("expr_pest", &hr_expr);
                code = code.replace("formatted_idents", formatted_idents);
                code = code.replace("character_condition", condition);
                if !empty_accepted {
                    code = code.replace("//NON-EMPTY", "");
                }
                return code
            }

            let mut code = G::pattern_expr_rep().to_owned();
            code = code.replace("expr_id", &id);
            code = code.replace("expr_pest", &hr_expr);
            code = code.replace("formatted_idents", formatted_idents);
            code = code.replace("inner_eid", &ids.id(expr));
            code = code.replace("inner_idents", match contains_idents(expr, has_whitespace) {
                true => "idents",
                false => "",
            });

            if has_whitespace {
                code = code.replace("//WSP", "");
            }
            if !empty_accepted {
                code = code.replace("//NON-EMPTY", "");
            }
            return code;
        }
        FPestExpr::Opt(expr) => {
            let code = G::pattern_expr_opt().to_owned();
            let code = code.replace("expr_id", &id);
            let code = code.replace("expr_pest", &hr_expr);
            let code = code.replace("formatted_idents", formatted_idents);
            let code = code.replace("inner_eid", &ids.id(expr));
            let code = code.replace("inner_idents", match contains_idents(expr, has_whitespace) {
                true => "idents",
                false => "",
            });
            code
        }
        FPestExpr::NegPred(expr) => {
            let code = G::pattern_expr_neg().to_owned();
            let code = code.replace("expr_id", &id);
            let code = code.replace("expr_pest", &hr_expr);
            let code = code.replace("formatted_idents", formatted_idents);
            let code = code.replace("inner_id", &ids.id(expr));
            let code = code.replace("inner_idents", match contains_idents(expr, has_whitespace) {
                true => "idents",
                false => "",
            });
            code
        }
        FPestExpr::Insens(value) => {
            let inverted_value = value.chars().map(|c| {
                if c.is_ascii_uppercase() {
                    c.to_ascii_lowercase()
                } else {
                    c.to_ascii_uppercase()
                }
            }).collect::<String>();

            let code = G::pattern_expr_insens().to_owned();
            let code = code.replace("expr_id", &id);
            let code = code.replace("expr_pest", &hr_expr);
            let code = code.replace("expr_str", format!("{value:?}").as_str());
            let code = code.replace("expr_inv_str", format!("{inverted_value:?}").as_str());
            let code = code.replace("expr_len_str", &value.len().to_string());
            code
        }
    }
}
