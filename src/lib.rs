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
    exprs.sort_by_key(|expr| ids.id(expr));
    exprs.dedup_by_key(|expr| ids.id(expr));
    exprs
}

trait HackTrait {
    fn code(&self, ids: &mut IdRegistry) -> String;
    fn output_ty(&self) -> String;
    fn output_ty_for_choice(&self) -> String;
}

impl HackTrait for OptimizedExpr {
    fn code(&self, ids: &mut IdRegistry) -> String {
        let id = ids.id(self);
        let output_ty = self.output_ty();
        let output_ty_nolt = output_ty.trim_end_matches("<'i>");
        match self {
            OptimizedExpr::Ident(ident) => {
                if ident == "ASCII_DIGIT" {
                    format!(r#"
                    fn parse_{id}<'i>(input: &'i str) -> Res<'i, {output_ty}> {{
                        if let Some(first) = input.chars().next() {{
                            if first.is_ascii_digit() {{
                                Ok((&input[1..], &input[..1]))
                            }} else {{
                                Err("nope")
                            }}
                        }} else {{
                            Err("nope")
                        }}
                    }}
                    "#)
                } else {
                    format!(r#"
                    IDENT {ident} ({id})
                    "#)
                }
            }
            OptimizedExpr::Choice(first, second) => {
                let first_id = ids.id(first);
                let first_ty_for_choice = first.output_ty_for_choice();
                let first_ty = first.output_ty();
                let second_id = ids.id(second);
                let second_ty_for_choice = second.output_ty_for_choice();
                let second_ty = second.output_ty();

                if first_ty != second_ty {
                    format!(r#"
                    enum {output_ty} {{
                        {first_ty_for_choice}({first_ty}),
                        {second_ty_for_choice}({second_ty}),
                    }}
    
                    fn parse_{id}<'i>(input: &'i str) -> Res<'i, {output_ty}> {{
                        if let Ok((input, res)) = parse_{first_id}(input) {{
                            Ok((input, {output_ty_nolt}::{first_ty_for_choice}(res)))
                        }} else if let Ok((input, res)) = parse_{second_id}(input) {{
                            Ok((input, {output_ty_nolt}::{second_ty_for_choice}(res)))
                        }} else {{
                            Err("nope")
                        }}
                    }}
                    "#)
                } else {
                    format!(r#"
                    fn parse_{id}<'i>(input: &'i str) -> Res<'i, {output_ty}> {{
                        if let Ok((input, res)) = parse_{first_id}(input) {{
                            Ok((input, res))
                        }} else if let Ok((input, res)) = parse_{second_id}(input) {{
                            Ok((input, res))
                        }} else {{
                            Err("nope")
                        }}
                    }}
                    "#)
                }
            }
            OptimizedExpr::Str(value) => {
                format!(r#"
                fn parse_{id}<'i>(input: &'i str) -> Res<'i, {output_ty}> {{
                    if input.starts_with("{value}") {{
                        Ok((&input["{value}".len()..], "{value}"))
                    }} else {{
                        Err("nope")
                    }}
                }}
                "#)
            }
            OptimizedExpr::Seq(first, second) => {
                let first_id = ids.id(first);
                let second_id = ids.id(second);
                format!(r#"
                fn parse_{id}<'i>(input: &'i str) -> Res<'i, {output_ty}> {{
                    let (input, res_{first_id}) = parse_{first_id}(input)?;
                    let (input, res_{second_id}) = parse_{second_id}(input)?;
                    Ok((input, (res_{first_id}, res_{second_id})))
                }}
                "#)
            }
            OptimizedExpr::Rep(expr) => {
                let expr_id = ids.id(expr);
                format!(r#"
                fn parse_{id}<'i>(mut input: &'i str) -> Res<'i, {output_ty}> {{
                    let mut res = Vec::new();
                    while let Ok((new_input, res_{expr_id})) = parse_{expr_id}(input) {{
                        input = new_input;
                        res.push(res_{expr_id});
                    }}
                    Ok((input, res))
                }}
                "#)
            }
            expr => todo!("code on {:?}", expr),
        }
    }

    fn output_ty(&self) -> String {
        match self {
            OptimizedExpr::Seq(first, second) => {
                let first_ty = first.output_ty();
                let second_ty = second.output_ty();
                format!("({first_ty}, {second_ty})")
            }
            OptimizedExpr::Choice(first, second) => {
                let first_ty = first.output_ty_for_choice();
                let second_ty = second.output_ty_for_choice();
                if first_ty == second_ty {
                    return first.output_ty();
                }
                format!("{first_ty}Or{second_ty}<'i>")
            }
            OptimizedExpr::Str(_)  => "&'i str".to_string(),
            OptimizedExpr::Ident(ident) => {
                if ident == "ASCII_DIGIT" {
                    return "&'i str".to_string();
                }
                let mut chars = ident.chars();
                let first = chars.next().unwrap().to_uppercase().to_string();
                let rest = chars.collect::<String>();
                format!("{}{}", first, rest)
            },
            OptimizedExpr::Rep(expr) => {
                let ty = expr.output_ty();
                format!("Vec<{ty}>")
            }
            expr => todo!("output ty on {:?}", expr),
        }
    }

    fn output_ty_for_choice(&self) -> String {
        match self {
            OptimizedExpr::Seq(first, second) => {
                let first_ty = first.output_ty_for_choice();
                let second_ty = second.output_ty_for_choice();
                format!("{first_ty}And{second_ty}")
            }
            OptimizedExpr::Choice(first, second) => {
                let first_ty = first.output_ty_for_choice();
                let second_ty = second.output_ty_for_choice();
                if first_ty == second_ty {
                    return first_ty;
                }
                format!("{first_ty}Or{second_ty}")
            }
            OptimizedExpr::Ident(ident) => {
                if ident == "ASCII_DIGIT" {
                    return "Str".to_string();
                }
                let mut chars = ident.chars();
                let first = chars.next().unwrap().to_uppercase().to_string();
                let rest = chars.collect::<String>();
                format!("{}{}", first, rest)
            },
            OptimizedExpr::Str(_) => "Str".to_string(),
            OptimizedExpr::Rep(expr) => {
                let ty = expr.output_ty();
                format!("Vec{ty}")
            }
            expr => todo!("choice output ty on {:?}", expr),
        }
    }
}

#[test]
fn test() {
    let grammar = r#"field = { (ASCII_DIGIT | "." | "-")+ }"#;
    let (_, rules) = pest_meta::parse_and_optimize(grammar).unwrap();
    println!("{:#?}", rules);
    for rule in rules {
        let mut ids = IdRegistry::new();
        let exprs = extract_exprs(&rule.expr, &mut ids);
        let mut full_code = String::new();
        full_code.push_str("type Res<'i, T> = Result<(&'i str, T), &'static str>;\n\n");
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
        let rule_name = rule.name.as_str();
        let top_expr_id = ids.id(&rule.expr);
        let top_output_ty = rule.expr.output_ty();
        full_code.push_str(&format!(r#"
        pub fn parse_{rule_name}<'i>(input: &'i str) -> Result<{top_output_ty}, &'static str> {{
            match parse_{top_expr_id}(input) {{
                Ok((_, res)) => Ok(res),
                Err(e) => Err(e),
            }}
        }}
        "#));
        println!("{full_code}");
    }
}
