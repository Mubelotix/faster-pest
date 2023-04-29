use std::collections::HashMap;

pub(crate) use pest_meta::{optimizer::OptimizedExpr, ast::RuleType};
extern crate proc_macro;
use proc_macro::TokenStream;

mod ids;
pub(crate) use ids::*;
mod tree_inspection;
pub(crate) use tree_inspection::*;
mod expr_codegen;
pub(crate) use expr_codegen::*;
mod optimizer;
pub(crate) use optimizer::*;

use syn::*;
use proc_macro2::TokenTree;

#[proc_macro_derive(Parser, attributes(grammar))]
pub fn derive_parser(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_ident = ast.ident;
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
    full_code.push_str(&format!("impl {struct_ident} {{\n"));
    full_code.push_str("    pub fn parse(rule: Rule, input: &str) -> Result<Pairs2<Ident>, Error> {\n");
    full_code.push_str("        let mut idents = Vec::with_capacity(500);\n"); // TODO: refine 500
    full_code.push_str("        match rule {\n");
    for rule in &rules {
        let name = rule.name.as_str();
        if !silent_rules.contains(&name) {
            full_code.push_str(&format!("            Rule::{name} => {struct_ident}_faster_pest::parse_{name}(input.as_bytes(), &mut idents)?,\n"));
        }
    }
    full_code.push_str("        };\n");
    full_code.push_str("        Ok(unsafe {{ Pairs2::from_idents(idents, input) }})\n");
    full_code.push_str("    }\n");
    full_code.push_str("}\n\n");

    full_code.push_str("\n\n#[automatically_derived]\n");
    full_code.push_str("#[allow(clippy::all)]\n");
    full_code.push_str(&format!("pub mod {struct_ident}_faster_pest {{\n"));
    full_code.push_str("    use super::*;\n");

    let mut ids = IdRegistry::new();
    let mut optimized_exprs = Vec::new();
    let mut exprs = Vec::new();
    let mut character_set_rules = HashMap::new();
    for rule in &rules {
        let expr = optimize(&rule.expr);
        if let FPestExpr::CharacterCondition(c) = &expr {
            character_set_rules.insert(rule.name.as_str(), c.to_owned());
        }
        optimized_exprs.push(expr);
    }
    for expr in &mut optimized_exprs {
        optimize_second_stage(expr, &character_set_rules);
    }
    println!("{:#?}", optimized_exprs);
    for (i, rule) in rules.iter().enumerate() {
        let expr = optimized_exprs.get(i).unwrap();
        exprs.extend(list_exprs(expr));
        let rule_name = rule.name.as_str();
        let rule_name_pascal_case = rule_name.chars().next().unwrap().to_uppercase().collect::<String>() + &rule_name[1..];
        let top_expr_id = ids.id(expr);
        let formatted_idents = match contains_idents(expr, has_whitespace) {
            true => "idents",
            false => "",
        };
        match silent_rules.contains(&rule_name) {
            false => full_code.push_str(&format!(r#"
                #[automatically_derived]
                #[allow(clippy::all)]
                pub fn parse_{rule_name}<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {{
                    let idents_len = idents.len();
                    if idents_len == idents.capacity() {{
                        idents.reserve(500);
                    }}
                    unsafe {{ idents.set_len(idents_len + 1); }}
                    let new_input = match parse_{top_expr_id}(input, {formatted_idents}) {{
                        Ok(input) => input,
                        Err(e) => {{
                            unsafe {{ idents.set_len(idents_len); }}
                            return Err(e);
                        }}
                    }};
                    let content = unsafe {{ std::str::from_utf8_unchecked(input.get_unchecked(..input.len() - new_input.len())) }};
                    unsafe {{ *idents.get_unchecked_mut(idents_len) = (Ident::{rule_name_pascal_case}(content), idents.len()); }}
                    Ok(new_input)
                }}

                #[automatically_derived]
                #[allow(clippy::all)]
                pub fn quick_parse_{rule_name}<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {{
                    let idents_len = idents.len();
                    if idents_len == idents.capacity() {{
                        idents.reserve(500);
                    }}
                    unsafe {{ idents.set_len(idents_len + 1); }}
                    let new_input = match quick_parse_{top_expr_id}(input, {formatted_idents}) {{
                        Some(input) => input,
                        None => {{
                            unsafe {{ idents.set_len(idents_len); }}
                            return None;
                        }}
                    }};
                    let content = unsafe {{ std::str::from_utf8_unchecked(input.get_unchecked(..input.len() - new_input.len())) }};
                    unsafe {{ *idents.get_unchecked_mut(idents_len) = (Ident::{rule_name_pascal_case}(content), idents.len()); }}
                    Some(new_input)
                }}
                "#)
            ),
            true => full_code.push_str(&format!(r#"
                #[automatically_derived]
                #[allow(clippy::all)]
                pub fn parse_{rule_name}<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Result<&'i [u8], Error> {{
                    parse_{top_expr_id}(input, {formatted_idents})
                }}

                #[automatically_derived]
                #[allow(clippy::all)]
                pub fn quick_parse_{rule_name}<'i, 'b>(input: &'i [u8], idents: &'b mut Vec<(Ident<'i>, usize)>) -> Option<&'i [u8]> {{
                    quick_parse_{top_expr_id}(input, {formatted_idents})
                }}
                "#)
            ),
        }

        full_code.push_str(&format!(r#"
            #[automatically_derived]
            #[allow(clippy::all)]
            impl {struct_ident} {{
                pub fn parse_{rule_name}(input: &str) -> Result<IdentList<Ident>, Error> {{
                    let mut idents = Vec::with_capacity(500);
                    if quick_parse_{rule_name}(input.as_bytes(), &mut idents).is_some() {{
                        return Ok(unsafe {{ IdentList::from_idents(idents) }});
                    }}
                    idents.clear();
                    parse_{rule_name}(input.as_bytes(), &mut idents)?;
                    Ok(unsafe {{ IdentList::from_idents(idents) }})
                }}
            }}"#
        ));
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
    full_code.push_str("}\n");
    std::fs::write("target/fp_code.rs", &full_code).unwrap();

    full_code.parse().unwrap()
}
