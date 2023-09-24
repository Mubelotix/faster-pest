use faster_pest_generator::Generator;
extern crate proc_macro;
use proc_macro::TokenStream;

use syn::*;
use proc_macro2::TokenTree;

fn list_grammar_files(attrs: &[Attribute]) -> Vec<String> {
    attrs.iter().filter(|attr| attr.path.is_ident("grammar")).map(|a| {
        let mut tokens = a.tokens.clone().into_iter();
        match tokens.next() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == '=' => (),
            _ => panic!("Expected leading '=' in grammar attribute"),
        }
        let path = match tokens.next() {
            Some(TokenTree::Literal(value)) => value.to_string(),
            _ => panic!("Expected literal in grammar attribute")
        };
        path.trim_matches('"').to_string()
    }).collect()
}

struct RustGenerator {

}

impl Generator for RustGenerator {
    fn ident(ident: &str) -> String {
        String::from(match ident {
            "EOI" => {
                r#"
                pub fn parse_expr_id<'i>(input: &'i [u8]) -> Result<&'i [u8], Error> {{
                    if input.is_empty() {{
                        Ok(input)
                    }} else {{
                        Err(Error::new(ErrorKind::Expected("EOI"), unsafe{{std::str::from_utf8_unchecked(input)}}, "EOI"))
                    }}
                }}
                pub fn quick_parse_expr_id<'i>(input: &'i [u8]) -> Option<&'i [u8]> {{
                    if input.is_empty() {{
                        Some(input)
                    }} else {{
                        None
                    }}
                }}
                "#
            },
            "SOI" => {
                r#" // TODO
                pub fn parse_expr_id<'i>(input: &'i [u8]) -> Result<&'i [u8], Error> {{
                    Ok(input)
                }}
                pub fn quick_parse_expr_id<'i>(input: &'i [u8]) -> Option<&'i [u8]> {{
                    Some(input)
                }}
                "#
            }
            "NEWLINE" => {
                r#"
                pub fn parse_expr_id<'i>(input: &'i [u8]) -> Result<&'i [u8], Error> {{
                    if input.starts_with(b"\r\n") {{
                        Ok(unsafe {{ input.get_unchecked(2..) }})
                    }} else if input.starts_with(b"\n") || input.starts_with(b"\r") {{
                        Ok(unsafe {{ input.get_unchecked(1..) }})
                    }} else {{
                        Err(Error::new(ErrorKind::Expected("newline"), unsafe{{std::str::from_utf8_unchecked(input)}}, "NEWLINE"))
                    }}
                }}
                pub fn quick_parse_expr_id<'i>(input: &'i [u8]) -> Option<&'i [u8]> {{
                    if input.starts_with(b"\r\n") {{
                        Some(unsafe {{ input.get_unchecked(2..) }})
                    }} else if input.starts_with(b"\n") || input.starts_with(b"\r") {{
                        Some(unsafe {{ input.get_unchecked(1..) }})
                    }} else {{
                        None
                    }}
                }}
                "#
            }
            _ => ""
        })

    }

    fn character_ident(ident: &str) -> Option<&'static str> {
        match ident {
            "ASCII_DIGIT" => Some("c.is_ascii_digit()"),
            "ASCII_NONZERO_DIGIT" => Some("(c.is_ascii_digit() && c != '0')"),
            "ASCII_ALPHA_LOWER" => Some("c.is_ascii_lowercase()"),
            "ASCII_ALPHA_UPPER" => Some("c.is_ascii_uppercase()"),
            "ASCII_ALPHA" => Some("c.is_ascii_alphabetic()"),
            "ASCII_ALPHANUMERIC" => Some("c.is_ascii_alphanumeric()"),
            "ASCII" => Some("c.is_ascii()"),
            "ANY" => Some("true"),
            _ => None
        }
    }

    fn character(c: u8) -> String {
        format!("(c == &{c})")
    }

    fn character_range(c1: u8, c2: u8) -> String {
        format!("(c >= &{c1} && c <= &{c2})")
    }

    fn pattern_expr_character() -> &'static str {
        include_str!("pattern_expr_character.rs")
    }

    fn pattern_expr_choice() -> &'static str {
        include_str!("pattern_expr_choice.rs")
    }

    fn pattern_expr_insens() -> &'static str {
        include_str!("pattern_expr_insens.rs")
    }

    fn pattern_expr_neg() -> &'static str {
        include_str!("pattern_expr_neg.rs")
    }

    fn pattern_expr_opt() -> &'static str {
        include_str!("pattern_expr_opt.rs")
    }

    fn pattern_expr_rep_character() -> &'static str {
        include_str!("pattern_expr_rep_character.rs")
    }

    fn pattern_expr_rep() -> &'static str {
        include_str!("pattern_expr_rep.rs")
    }

    fn pattern_expr_seq() -> &'static str {
        include_str!("pattern_expr_seq.rs")
    }

    fn pattern_expr_str() -> &'static str {
        include_str!("pattern_expr_str.rs")
    }

    fn pattern_outer() -> &'static str {
        include_str!("pattern_outer.rs")
    }

    fn pattern_rule_method() -> &'static str {
        include_str!("pattern_rule_method.rs")
    }

    fn pattern_rule_silent() -> &'static str {
        include_str!("pattern_rule_silent.rs")
    }

    fn pattern_rule() -> &'static str {
        include_str!("pattern_rule.rs")
    }
}

#[proc_macro_derive(Parser, attributes(grammar))]
pub fn derive_parser(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_ident = ast.ident;

    let grammar_files = list_grammar_files(&ast.attrs);

    let code = faster_pest_generator::gen::<RustGenerator>(struct_ident.to_string(), grammar_files);

    std::fs::write("target/fp_code.rs", &code).unwrap();
    
    code.parse().unwrap()
}
