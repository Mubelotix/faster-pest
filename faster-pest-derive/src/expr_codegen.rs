use crate::{*, optimizer::FPestExpr};

pub const CONDITIONS: &[(&str, &str)] = &[
    ("ASCII_DIGIT", "c.is_ascii_digit()"),
    ("ASCII_NONZERO_DIGIT", "(c.is_ascii_digit() && c != '0')"),
    ("ASCII_ALPHA_LOWER", "c.is_ascii_lowercase()"),
    ("ASCII_ALPHA_UPPER", "c.is_ascii_uppercase()"),
    ("ASCII_ALPHA", "c.is_ascii_alphabetic()"),
    ("ASCII_ALPHANUMERIC", "c.is_ascii_alphanumeric()"),
    ("ASCII", "c.is_ascii()"),
    ("ANY", "true"),
];

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

pub fn code(expr: &FPestExpr, ids: &mut IdRegistry, has_whitespace: bool) -> String {
    let id = ids.id(expr);
    let formatted_idents = match contains_idents(expr, has_whitespace) {
        true => "idents: &'b mut Vec<(Ident<'i>, usize)>",
        false => "",
    };
    let hr_expr = to_pest(expr);
    let hr_expre = hr_expr.replace('\\', "\\\\").replace('\"', "\\\"");
    match expr {
        FPestExpr::Ident(ident) => {
            match ident.as_str() {
                "EOI" => {
                    format!(r#"
                    pub fn parse_{id}<'i>(input: &'i [u8]) -> Result<&'i [u8], Error> {{
                        if input.is_empty() {{
                            Ok(input)
                        }} else {{
                            Err(Error::new(ErrorKind::Expected("EOI"), unsafe{{std::str::from_utf8_unchecked(input)}}, "EOI"))
                        }}
                    }}
                    pub fn quick_parse_{id}<'i>(input: &'i [u8]) -> Option<&'i [u8]> {{
                        if input.is_empty() {{
                            Some(input)
                        }} else {{
                            None
                        }}
                    }}

                    "#)
                }
                "SOI" => {
                    format!(r#" // TODO
                    pub fn parse_{id}<'i>(input: &'i [u8]) -> Result<&'i [u8], Error> {{
                        Ok(input)
                    }}
                    pub fn quick_parse_{id}<'i>(input: &'i [u8]) -> Option<&'i [u8]> {{
                        Some(input)
                    }}

                    "#)
                }
                "NEWLINE" => {
                    format!(r#"
                    pub fn parse_{id}<'i>(input: &'i [u8]) -> Result<&'i [u8], Error> {{
                        if input.starts_with(b"\r\n") {{
                            Ok(unsafe {{ input.get_unchecked(2..) }})
                        }} else if input.starts_with(b"\n") || input.starts_with(b"\r") {{
                            Ok(unsafe {{ input.get_unchecked(1..) }})
                        }} else {{
                            Err(Error::new(ErrorKind::Expected("newline"), unsafe{{std::str::from_utf8_unchecked(input)}}, "NEWLINE"))
                        }}
                    }}
                    pub fn quick_parse_{id}<'i>(input: &'i [u8]) -> Option<&'i [u8]> {{
                        if input.starts_with(b"\r\n") {{
                            Some(unsafe {{ input.get_unchecked(2..) }})
                        }} else if input.starts_with(b"\n") || input.starts_with(b"\r") {{
                            Some(unsafe {{ input.get_unchecked(1..) }})
                        }} else {{
                            None
                        }}
                    }}

                    "#)
                }
                _ => String::new()
            }
        }
        FPestExpr::CharacterCondition(condition) => {
            let mut code = include_str!("pattern_expr_character.rs").to_owned();
            code = code.replace("expr_id", &id);
            code = code.replace("expr_pest", &hr_expr);
            code = code.replace("formatted_idents", formatted_idents);
            code = code.replace("character_condition", condition);
            code
        }
        FPestExpr::Choice(items) => {
            let mut code = include_str!("pattern_expr_choice.rs").to_owned();
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
            let mut code = include_str!("pattern_expr_str.rs").to_owned();
            code = code.replace("expr_id", &id);
            code = code.replace("expr_pest", &hr_expr);
            code = code.replace("expr_str", format!("{value:?}").as_str());
            code
        }
        FPestExpr::Seq(items) => {
            let mut code = include_str!("pattern_expr_seq.rs").to_owned();
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
                let mut code = include_str!("pattern_expr_rep_character.rs").to_owned();
                code = code.replace("expr_id", &id);
                code = code.replace("expr_pest", &hr_expr);
                code = code.replace("formatted_idents", formatted_idents);
                code = code.replace("character_condition", condition);
                if !empty_accepted {
                    code = code.replace("//NON-EMPTY", "");
                }
                return code
            }

            let mut code = include_str!("pattern_expr_rep.rs").to_owned();
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
            let code = include_str!("pattern_expr_opt.rs").to_owned();
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
            let code = include_str!("pattern_expr_neg.rs").to_owned();
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

            let code = include_str!("pattern_expr_insens.rs").to_owned();
            let code = code.replace("expr_id", &id);
            let code = code.replace("expr_pest", &hr_expr);
            let code = code.replace("expr_str", format!("{value:?}").as_str());
            let code = code.replace("expr_inv_str", format!("{inverted_value:?}").as_str());
            let code = code.replace("expr_len_str", &value.len().to_string());
            code
        }
    }
}
