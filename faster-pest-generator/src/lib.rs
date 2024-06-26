use std::collections::HashMap;

use pest_meta::optimizer::OptimizedRule;
pub(crate) use pest_meta::{optimizer::OptimizedExpr, ast::RuleType};

mod ids;
pub(crate) use ids::*;
mod tree_inspection;
pub(crate) use tree_inspection::*;
mod expr_codegen;
pub(crate) use expr_codegen::*;
mod optimizer;
pub(crate) use optimizer::*;

pub trait Generator {
    fn ident(ident: &str) -> String;
    fn character_ident(ident: &str) -> Option<&'static str>;
    fn character(c: u8) -> String;
    fn character_range(c1: u8, c2: u8) -> String;
    fn pattern_expr_character() -> &'static str;
    fn pattern_expr_choice() -> &'static str;
    fn pattern_expr_insens() -> &'static str;
    fn pattern_expr_neg() -> &'static str;
    fn pattern_expr_opt() -> &'static str;
    fn pattern_expr_rep_character() -> &'static str;
    fn pattern_expr_rep() -> &'static str;
    fn pattern_expr_seq() -> &'static str;
    fn pattern_expr_str() -> &'static str;
    fn pattern_outer() -> &'static str;
    fn pattern_rule_method() -> &'static str;
    fn pattern_rule_silent() -> &'static str;
    fn pattern_rule() -> &'static str;
}

fn multi_replace(mut text: String, values: Vec<(&'static str, Vec<String>)>) -> String {
    assert!(!values.is_empty(), "Patterns and values must not be empty.");
    assert!(values.iter().all(|v| v.1.len() == values[0].1.len()), "Values must equal lenghts.");
    assert!(!values.iter().any(|v| values.iter().any(|other| other.0 != v.0 && other.0.contains(v.0))), "Patterns must not contain each other.");

    let mut line_ranges = Vec::new();
    for line in text.lines() {
        let begin = line.as_ptr() as usize - text.as_ptr() as usize;
        let end = begin + line.len();
        line_ranges.push(begin..end);
    }

    for line in line_ranges.into_iter().rev() {
        let mut is_to_be_replaced = false;
        for (pattern, _) in &values {
            if text[line.clone()].contains(pattern) {
                is_to_be_replaced = true;
                break;
            }
        }
        if !is_to_be_replaced {
            continue;
        }

        let mut new_lines: Vec<String> = Vec::new();
        for i in 0..values[0].1.len() {
            let mut new_line = text[line.clone()].to_string();
            for (pattern, values) in &values {
                new_line = new_line.replace(pattern, &values[i]);
            }
            new_lines.push(new_line);
        }

        if new_lines.is_empty() {
            text.replace_range(line.start..line.end+1, "");
        } else {
            text.replace_range(line.clone(), new_lines.join("\n").as_str());
        }
    }
    
    text
}

fn get_all_rules(grammar_files: &[String]) -> Vec<OptimizedRule> {
    let mut rules = HashMap::new();

    for path in grammar_files {
        let Ok(grammar) = std::fs::read_to_string(path) else {
            panic!("Could not read grammar file at {path:?}");
        };
        let (_, new_rules) = match pest_meta::parse_and_optimize(&grammar) {
            Ok(new_rules) => new_rules,
            Err(e) => panic!("{}", e[0])
        };
        for new_rule in new_rules {
            rules.insert(new_rule.name.clone(), new_rule);
        }
    }

    let mut rules: Vec<OptimizedRule> = rules.into_values().collect();
    rules.sort_by_key(|rule| rule.name.clone());
    rules
}

pub fn gen<G: Generator>(struct_ident: String, grammar_files: Vec<String>) -> String {
    let rules = get_all_rules(&grammar_files);

    // Find silent rules
    let silent_rules = rules.iter().filter(|rule| matches!(rule.ty, RuleType::Silent)).map(|rule| rule.name.as_str()).collect::<Vec<_>>();

    // Find if there is a rule named WHITESPACE
    let has_whitespace = rules.iter().any(|rule| rule.name.as_str() == "WHITESPACE");

    let mut full_code = G::pattern_outer().to_string();
    full_code = multi_replace(full_code, vec![
        ("RuleVariant", rules.iter().filter(|r| !silent_rules.contains(&r.name.as_str())).map(|rule| rule.name.as_str().to_string()).collect()),
        ("IdentVariant", rules.iter().filter(|r| !silent_rules.contains(&r.name.as_str())).map(|rule| {
            let name = rule.name.as_str();
            let name_pascal_case = name.chars()
                .next()
                .expect("Rule name must not be empty")
                .to_uppercase()
                .collect::<String>()
                + &name[1..];
            name_pascal_case
        }).collect()),
    ]);
    full_code = full_code.replace("StructIdent", struct_ident.to_string().as_str());

    let mut ids = IdRegistry::new();
    let mut optimized_exprs = Vec::new();
    let mut exprs = Vec::new();
    let mut character_set_rules = HashMap::new();
    for rule in &rules {
        let expr = optimize::<G>(&rule.expr);
        if matches!(rule.ty, RuleType::Silent) {
            if let FPestExpr::CharacterCondition(c) = &expr {
                character_set_rules.insert(rule.name.as_str(), c.to_owned());
            }
        }
        optimized_exprs.push(expr);
    }
    for expr in &mut optimized_exprs {
        optimize_second_stage(expr, &character_set_rules);
    }
    println!("{:#?}", optimized_exprs);
    let mut inner_code = String::new();
    for (i, rule) in rules.iter().enumerate() {
        let expr = optimized_exprs.get(i).expect("Expr not found");
        exprs.extend(list_exprs(expr));
        let rule_name = rule.name.as_str();
        let rule_name_pascal_case = rule_name.chars()
            .next()
            .expect("Rule name must not be empty")
            .to_uppercase()
            .collect::<String>()
            + &rule_name[1..];
        let top_expr_id = ids.id(expr);
        let formatted_idents = match contains_idents(expr, has_whitespace) {
            true => "idents",
            false => "",
        };

        let mut code = match silent_rules.contains(&rule_name) {
            false => G::pattern_rule().to_string(),
            true => G::pattern_rule_silent().to_string(),
        };
        code.push_str(G::pattern_rule_method());
        code = code.replace("RuleVariant", rule.name.as_str());
        code = code.replace("top_expr_id", top_expr_id.to_string().as_str());
        code = code.replace("formatted_idents", formatted_idents);
        code = code.replace("IdentVariant", rule_name_pascal_case.as_str());
        code = code.replace("StructIdent", struct_ident.to_string().as_str());
        inner_code.push_str(code.as_str());
    }
    exprs.sort_by_key(|expr| ids.id(expr));
    exprs.dedup();
    for expr in exprs {
        let mut new_code = code::<G>(expr, &mut ids, has_whitespace);
        let mut new_code2 = new_code.trim_start_matches('\n');
        let new_code2_len = new_code2.len();
        new_code2 = new_code2.trim_start_matches(' ');
        let len_diff = new_code2_len - new_code2.len();
        let pattern = "\n".to_string() + &" ".repeat(len_diff);
        new_code = new_code.replace(&pattern, "\n");
        inner_code.push_str(new_code.as_str());
    }
    full_code = full_code.replace("    // inner code", inner_code.as_str());
    
    full_code.parse().expect("Unable to parse code")
}
