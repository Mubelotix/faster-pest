use std::{collections::HashMap, hash::Hash};
use pest_meta::optimizer::OptimizedExpr;

struct IdRegistry {
    ids: HashMap<String, usize>,
    next: usize,
}

impl IdRegistry {
    fn new() -> Self {
        Self {
            ids: HashMap::new(),
            next: 0,
        }
    }

    fn id(&mut self, expr: &OptimizedExpr) -> String {
        match expr {
            OptimizedExpr::Ident(ident) => ident.to_string(),
            expr => {
                let id = format!("{:?}", expr);
                let id = self.ids.entry(id).or_insert_with(|| {
                    let id = self.next;
                    self.next += 1;
                    id
                });
                format!("anon_{id}")
            }
        }
    }
}

fn extract_exprs<'a, 'b>(expr: &'a OptimizedExpr, ids: &'b mut IdRegistry) -> Vec<&'a OptimizedExpr> {
    let mut exprs = Vec::new();
    match expr {
        OptimizedExpr::PosPred(expr) | OptimizedExpr::NegPred(expr) | OptimizedExpr::Opt(expr) | OptimizedExpr::Rep(expr) | OptimizedExpr::Push(expr) | OptimizedExpr::RestoreOnErr(expr) => exprs.extend(extract_exprs(expr, ids)),
        OptimizedExpr::Seq(first, second) | OptimizedExpr::Choice(first, second) => {
            exprs.extend(extract_exprs(first, ids));
            exprs.extend(extract_exprs(second, ids));
        }
        _ => ()
    }
    exprs.push(expr);
    exprs
}

fn contains_idents(expr: &OptimizedExpr) -> bool {
    match expr {
        OptimizedExpr::Ident(ident) if ident != "ASCII_DIGIT" && ident != "SOI" && ident != "EOI" && ident != "NEWLINE" && ident != "ASCII_ALPHANUMERIC" => {
            true
        },
        OptimizedExpr::PosPred(expr) | OptimizedExpr::NegPred(expr) | OptimizedExpr::Opt(expr) | OptimizedExpr::Rep(expr) | OptimizedExpr::Push(expr) | OptimizedExpr::RestoreOnErr(expr) => contains_idents(expr),
        OptimizedExpr::Seq(first, second) | OptimizedExpr::Choice(first, second) => contains_idents(first) || contains_idents(second),
        _ => false
    }
}

trait HackTrait {
    fn code(&self, ids: &mut IdRegistry) -> String;
}

impl HackTrait for OptimizedExpr {
    fn code(&self, ids: &mut IdRegistry) -> String {
        let id = ids.id(self);
        let formatted_idents = match contains_idents(self) {
            true => "idents: &'b mut Vec<Ident<'i>>",
            false => "",
        };
        let (cancel1, cancel2, idents) = match contains_idents(self) {
            true => ("let idents_len = idents.len();", "idents.truncate(idents_len);", "idents"),
            false => ("", "", ""),
        };
        match self {
            OptimizedExpr::Ident(ident) => {
                match ident.as_str() {
                    "ASCII_DIGIT" => {
                        format!(r#"
                        fn parse_{id}<'i, 'b>(input: &'i str) -> Res<'i> {{
                            if let Some(first) = input.chars().next() {{
                                if first.is_ascii_digit() {{
                                    Ok(&input[1..])
                                }} else {{
                                    Err(Error{{desc: Error{{desc: "Expected an ASCII digit", input}})
                                }}
                            }} else {{
                                Err(Error{{desc: "Expected an ASCII digit, got EOI", input}})
                            }}
                        }}
                        "#)
                    }
                    "ASCII_ALPHANUMERIC" => {
                        format!(r#"
                        fn parse_{id}<'i, 'b>(input: &'i str) -> Res<'i> {{
                            if let Some(first) = input.chars().next() {{
                                if first.is_ascii_alphanumeric() {{
                                    Ok(&input[1..])
                                }} else {{
                                    Err(Error{{desc: "Expected an ASCII alphanumeric", input}})
                                }}
                            }} else {{
                                Err(Error{{desc: "Expected an ASCII alphanumeric, got EOI", input}})
                            }}
                        }}
                        "#)
                    }
                    "EOI" => {
                        format!(r#"
                        fn parse_{id}<'i, 'b>(input: &'i str) -> Res<'i> {{
                            if input.is_empty() {{
                                Ok(input)
                            }} else {{
                                Err(Error{{desc: "Expected EOI", input}})
                            }}
                        }}
                        "#)
                    }
                    "SOI" => {
                        format!(r#" // TODO
                        fn parse_{id}<'i, 'b>(input: &'i str) -> Res<'i> {{
                            Ok(input)
                        }}
                        "#)
                    }
                    "NEWLINE" => {
                        format!(r#"
                        fn parse_{id}<'i, 'b>(input: &'i str) -> Res<'i> {{
                            if input.starts_with("\r\n") {{
                                Ok(&input[2..])
                            }} else if input.starts_with("\n") {{
                                Ok(&input[1..])
                            }} else {{
                                Err(Error{{desc: "Expected newline", input}})
                            }}
                        }}
                        "#)
                    }
                    _ => String::new()
                }
            }
            OptimizedExpr::Choice(first, second) => {
                let first_id = ids.id(first);
                let first_idents = match contains_idents(first) {
                    true => "idents",
                    false => "",
                };
                let second_id = ids.id(second);
                let second_idents = match contains_idents(second) {
                    true => "idents",
                    false => "",
                };

                format!(r#"
                fn parse_{id}<'i, 'b>(input: &'i str, {formatted_idents}) -> Res<'i> {{
                    {cancel1}
                    if let Ok(input) = parse_{first_id}(input, {first_idents}) {{
                        return Ok(input);
                    }}
                    {cancel2}
                    {cancel1}
                    if let Ok(input) = parse_{second_id}(input, {second_idents}) {{
                        return Ok(input);
                    }}
                    {cancel2}
                    Err(Error{{desc: "Expected either {first_id} or {second_id}", input}})
                }}
                "#)
            }
            OptimizedExpr::Str(value) => {
                format!(r#"
                fn parse_{id}<'i, 'b>(input: &'i str) -> Res<'i> {{
                    if input.starts_with({value:?}) {{
                        Ok(&input[{value:?}.len()..])
                    }} else {{
                        Err(Error{{desc: "Expected '{value}'", input}})
                    }}
                }}
                "#)
            }
            OptimizedExpr::Seq(first, second) => {
                let first_id = ids.id(first);
                let first_idents = match contains_idents(first) {
                    true => "idents",
                    false => "",
                };
                let second_id = ids.id(second);
                let second_idents = match contains_idents(second) {
                    true => "idents",
                    false => "",
                };

                format!(r#"
                fn parse_{id}<'i, 'b>(mut input: &'i str, {formatted_idents}) -> Res<'i> {{
                    input = parse_{first_id}(input, {first_idents})?;
                    input = parse_{second_id}(input, {second_idents})?;
                    Ok(input)
                }}
                "#)
            }
            OptimizedExpr::Rep(expr) => {
                let expr_id = ids.id(expr);
                let idents = match contains_idents(expr) {
                    true => "idents",
                    false => "",
                };

                format!(r#"
                fn parse_{id}<'i, 'b>(mut input: &'i str, {formatted_idents}) -> Res<'i> {{
                    while let Ok(new_input) = parse_{expr_id}(input, {idents}) {{
                        input = new_input;
                    }}
                    Ok(input)
                }}
                "#)
            }
            OptimizedExpr::Opt(expr) => {
                let expr_id = ids.id(expr);

                format!(r#"
                fn parse_{id}<'i, 'b>(input: &'i str, {formatted_idents}) -> Res<'i> {{
                    {cancel1}
                    if let Ok(input) = parse_{expr_id}(input, {idents}) {{
                        Ok(input)
                    }} else {{
                        {cancel2}
                        Ok(input)
                    }}
                }}
                "#)
            }
            expr => todo!("code on {:?}", expr),
        }
    }
}


#[test]
fn test() {
    let grammar = include_str!("grammar.pest");
    let (_, rules) = pest_meta::parse_and_optimize(grammar).unwrap();
    println!("{:#?}", rules);
    let mut full_code = String::new();
    full_code.push_str(r#"
    #[derive(Debug)]
    struct Error<'i> {
        desc: &'static str,
        input: &'i str,
    }

    type Res<'i> = Result<&'i str, Error<'i>>;
    "#);

    // Create Ident enum
    full_code.push_str("#[derive(Debug)]\n");
    full_code.push_str("pub enum Ident<'i> {\n");
    for rule in &rules {
        let name = rule.name.as_str();
        let name_pascal_case = name.chars().next().unwrap().to_uppercase().collect::<String>() + &name[1..];
        full_code.push_str(&format!("    {name_pascal_case}(&'i str),\n"));
    }
    full_code.push_str("}\n\n");

    let mut ids = IdRegistry::new();
    let mut exprs = Vec::new();
    for rule in &rules {
        exprs.extend(extract_exprs(&rule.expr, &mut ids));
        let rule_name = rule.name.as_str();
        let rule_name_pascal_case = rule_name.chars().next().unwrap().to_uppercase().collect::<String>() + &rule_name[1..];
        let top_expr_id = ids.id(&rule.expr);
        let formatted_idents = match contains_idents(&rule.expr) {
            true => "idents",
            false => "",
        };
        full_code.push_str(&format!(r#"
        fn parse_{rule_name}<'i, 'b>(input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Res<'i> {{
            let i = idents.len();
            idents.push(Ident::{rule_name_pascal_case}(""));
            let new_input = match parse_{top_expr_id}(input, {formatted_idents}) {{
                Ok(input) => input,
                Err(e) => {{
                    idents.pop();
                    return Err(e);
                }}
            }};
            let new_ident = &input[..input.len() - new_input.len()];
            idents[i] = Ident::{rule_name_pascal_case}(new_ident);
            Ok(new_input)
        }}
        "#));
    }
    exprs.sort_by_key(|expr| ids.id(expr));
    exprs.dedup();
    for expr in exprs {
        let mut new_code = expr.code(&mut ids);
        let mut new_code2 = new_code.trim_start_matches('\n');
        let new_code2_len = new_code2.len();
        new_code2 = new_code2.trim_start_matches(' ');
        let len_diff = new_code2_len - new_code2.len();
        let pattern = "\n".to_string() + &" ".repeat(len_diff);
        new_code = new_code.replace(&pattern, "\n");
        full_code.push_str(new_code.as_str());
    }
    println!("{full_code}");
}
