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
}

impl HackTrait for OptimizedExpr {
    fn code(&self, ids: &mut IdRegistry) -> String {
        let id = ids.id(self);
        match self {
            OptimizedExpr::Ident(ident) => {
                if ident == "ASCII_DIGIT" {
                    format!(r#"
                    fn parse_{id}<'i>(input: &'i str) -> Res<'i> {{
                        if let Some(first) = input.chars().next() {{
                            if first.is_ascii_digit() {{
                                Ok(&input[1..])
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
                let second_id = ids.id(second);

                format!(r#"
                fn parse_{id}<'i>(input: &'i str) -> Res<'i> {{
                    if let Ok(input) = parse_{first_id}(input) {{
                        Ok(input)
                    }} else if let Ok(input) = parse_{second_id}(input) {{
                        Ok(input)
                    }} else {{
                        Err("nope")
                    }}
                }}
                "#)
            }
            OptimizedExpr::Str(value) => {
                format!(r#"
                fn parse_{id}<'i>(input: &'i str) -> Res<'i> {{
                    if input.starts_with("{value}") {{
                        Ok(&input["{value}".len()..])
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
                fn parse_{id}<'i>(mut input: &'i str) -> Res<'i> {{
                    input = parse_{first_id}(input)?;
                    input = parse_{second_id}(input)?;
                    Ok(input)
                }}
                "#)
            }
            OptimizedExpr::Rep(expr) => {
                let expr_id = ids.id(expr);
                format!(r#"
                fn parse_{id}<'i>(mut input: &'i str) -> Res<'i> {{
                    while let Ok(new_input) = parse_{expr_id}(input) {{
                        input = new_input;
                    }}
                    Ok(input)
                }}
                "#)
            }
            expr => todo!("code on {:?}", expr),
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
        full_code.push_str("type Res<'i> = Result<&'i str, &'static str>;\n\n");
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
        full_code.push_str(&format!(r#"
        pub fn parse_{rule_name}<'i>(input: &'i str) -> Result<&'i str, &'static str> {{
            match parse_{top_expr_id}(input) {{
                Ok(i) => Ok(i),
                Err(e) => Err(e),
            }}
        }}
        "#));
        println!("{full_code}");
    }
}
