pub(crate) use pest_meta::{optimizer::OptimizedExpr, ast::RuleType};
extern crate proc_macro;
use proc_macro::TokenStream;

mod ids;
pub(crate) use ids::*;
mod tree_inspection;
pub(crate) use tree_inspection::*;
mod expr_codegen;
pub(crate) use expr_codegen::*;

use syn::*;
use proc_macro2::TokenTree;

#[proc_macro_derive(Parser, attributes(grammar))]
pub fn derive_parser(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let mut grammar_tokens = ast.attrs.iter().find(|attr| attr.path.is_ident("grammar")).unwrap().tokens.clone().into_iter();
    match grammar_tokens.next() {
        Some(TokenTree::Punct(punct)) if punct.as_char() == '=' => (),
        _ => panic!("Expected leading '=' in grammar attribute"),
    }
    let grammar_path = match grammar_tokens.next() {
        Some(TokenTree::Literal(value)) => value.to_string(),
        _ => panic!("Expected literal in grammar attribute")
    };
    let grammar_path = grammar_path.trim_matches('"');

    let Ok(grammar) = std::fs::read_to_string(grammar_path) else {
        panic!("Could not read grammar file at {grammar_path:?}");
    };
    let (_, rules) = match pest_meta::parse_and_optimize(&grammar) {
        Ok(rules) => rules,
        Err(e) => panic!("{}", e[0])
    };
    println!("{:#?}", rules);
    let mut full_code = String::new();

    // Find silent rules
    let silent_rules = rules.iter().filter(|rule| matches!(rule.ty, RuleType::Silent)).map(|rule| rule.name.as_str()).collect::<Vec<_>>();

    // Find if there is a rule named WHITESPACE
    let has_whitespace = rules.iter().any(|rule| rule.name.as_str() == "WHITESPACE");

    // Create Ident enum
    full_code.push_str("#[derive(Debug, Copy, Clone)]\n");
    full_code.push_str("pub enum Ident<'i> {\n");
    for rule in &rules {
        let name = rule.name.as_str();
        if !silent_rules.contains(&name) {
            let name_pascal_case = name.chars().next().unwrap().to_uppercase().collect::<String>() + &name[1..];
            full_code.push_str(&format!("    {name_pascal_case}(&'i str),\n"));
        }
    }
    full_code.push_str("}\n");
    full_code.push_str("impl<'i> IdentTrait for Ident<'i> {\n");
    full_code.push_str("    type Rule = Rule;\n");
    full_code.push_str("    \n");
    full_code.push_str("    fn as_rule(&self) -> Rule {\n");
    full_code.push_str("        match self {\n");
    for rule in &rules {
        let name = rule.name.as_str();
        if !silent_rules.contains(&name) {
            let name_pascal_case = name.chars().next().unwrap().to_uppercase().collect::<String>() + &name[1..];
            full_code.push_str(&format!("            Ident::{name_pascal_case}(_) => Rule::{name},\n"));
        }
    }
    full_code.push_str("        }\n");
    full_code.push_str("    }\n");
    full_code.push_str("    \n");
    full_code.push_str("    fn as_str(&self) -> &str {\n");
    full_code.push_str("        match self {\n");
    for rule in &rules {
        let name = rule.name.as_str();
        if !silent_rules.contains(&name) {
            let name_pascal_case = name.chars().next().unwrap().to_uppercase().collect::<String>() + &name[1..];
            full_code.push_str(&format!("            Ident::{name_pascal_case}(s) => s,\n"));
        }
    }
    full_code.push_str("        }\n");
    full_code.push_str("    }\n");
    full_code.push_str("}\n\n");

    // Create Rule enum
    full_code.push_str("#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]\n");
    full_code.push_str("pub enum Rule {\n");
    for rule in &rules {
        let name = rule.name.as_str();
        if !silent_rules.contains(&name) {
            full_code.push_str(&format!("    {name},\n"));
        }
    }
    full_code.push_str("}\n\n");

    // Create parse method TODO name
    full_code.push_str("#[automatically_derived]\n");
    full_code.push_str(&format!("impl {} {{\n", ast.ident));
    full_code.push_str("    pub fn parse(rule: Rule, input: &str) -> Result<Pairs2<Ident>, pest::error::Error<Rule>> {\n");
    full_code.push_str("        let mut idents = Vec::new();\n");
    full_code.push_str("        match rule {\n");
    for rule in &rules {
        let name = rule.name.as_str();
        if !silent_rules.contains(&name) {
            full_code.push_str(&format!("            Rule::{name} => parse_{name}(input, &mut idents).map_err(|e| e.into_pest(input))?,\n"));
        }
    }
    full_code.push_str("        };\n");
    full_code.push_str("        Ok(Pairs2::from_idents(idents, input))\n");
    full_code.push_str("    }\n");
    full_code.push_str("}\n\n");

    let mut ids = IdRegistry::new();
    let mut exprs = Vec::new();
    for rule in &rules {
        exprs.extend(list_exprs(&rule.expr, false));
        let rule_name = rule.name.as_str();
        let rule_name_pascal_case = rule_name.chars().next().unwrap().to_uppercase().collect::<String>() + &rule_name[1..];
        let top_expr_id = ids.id(&rule.expr);
        let formatted_idents = match contains_idents(&rule.expr, has_whitespace) {
            true => "idents",
            false => "",
        };
        match silent_rules.contains(&rule_name) {
            false => full_code.push_str(&format!(r#"
                fn parse_{rule_name}<'i, 'b>(input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Result<&'i str, Error> {{
                    let idents_len = idents.len();
                    idents.push(Ident::{rule_name_pascal_case}(""));
                    let new_input = match parse_{top_expr_id}(input, {formatted_idents}) {{
                        Ok(input) => input,
                        Err(e) => {{
                            idents.truncate(idents_len);
                            return Err(e);
                        }}
                    }};
                    let new_ident = &input[..input.len() - new_input.len()];
                    idents[idents_len] = Ident::{rule_name_pascal_case}(new_ident);
                    Ok(new_input)
                }}

                fn quick_parse_{rule_name}<'i, 'b>(input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Option<&'i str> {{
                    let idents_len = idents.len();
                    idents.push(Ident::{rule_name_pascal_case}(""));
                    let new_input = match quick_parse_{top_expr_id}(input, {formatted_idents}) {{
                        Some(input) => input,
                        None => {{
                            idents.truncate(idents_len);
                            return None;
                        }}
                    }};
                    let new_ident = &input[..input.len() - new_input.len()];
                    idents[idents_len] = Ident::{rule_name_pascal_case}(new_ident);
                    Some(new_input)
                }}
                "#)
            ),
            true => full_code.push_str(&format!(r#"
                fn parse_{rule_name}<'i, 'b>(input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Result<&'i str, Error> {{
                    parse_{top_expr_id}(input, {formatted_idents})
                }}

                fn quick_parse_{rule_name}<'i, 'b>(input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Option<&'i str> {{
                    quick_parse_{top_expr_id}(input, {formatted_idents})
                }}
                "#)
            ),
        }
    }
    exprs.sort_by_key(|expr| ids.id(expr));
    exprs.dedup();
    for expr in exprs {
        let mut new_code = code(expr, &mut ids, has_whitespace);
        let mut new_code2 = new_code.trim_start_matches('\n');
        let new_code2_len = new_code2.len();
        new_code2 = new_code2.trim_start_matches(' ');
        let len_diff = new_code2_len - new_code2.len();
        let pattern = "\n".to_string() + &" ".repeat(len_diff);
        new_code = new_code.replace(&pattern, "\n");
        full_code.push_str(new_code.as_str());
    }
    println!("{full_code}");

    full_code.parse().unwrap()
}
