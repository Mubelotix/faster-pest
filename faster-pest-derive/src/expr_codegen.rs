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
    let (cancel1, cancel2, idents) = match contains_idents(expr, has_whitespace) {
        true => ("let idents_len = idents.len();", "unsafe { idents.set_len(idents_len); }", "idents"),
        false => ("", "", ""),
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
            format!(r#"
            // {condition}
            pub fn parse_{id}<'i>(input: &'i [u8]) -> Result<&'i [u8], Error> {{
                if !input.is_empty() {{
                    let c = unsafe {{ input.get_unchecked(0) }};
                    if {condition} {{
                        Ok(unsafe {{ input.get_unchecked(1..) }})
                    }} else {{
                        Err(Error::new(ErrorKind::Expected("unknown"), unsafe{{std::str::from_utf8_unchecked(input)}}, "{id} {hr_expre}")) // TODO: remove unknown
                    }}
                }} else {{
                    Err(Error::new(ErrorKind::Expected("unknown"), unsafe{{std::str::from_utf8_unchecked(input)}}, "{id} {hr_expre}"))
                }}
            }}
            pub fn quick_parse_{id}<'i>(input: &'i [u8]) -> Option<&'i [u8]> {{
                if !input.is_empty() {{
                    let c = unsafe {{ input.get_unchecked(0) }};
                    if {condition} {{
                        Some(unsafe {{ input.get_unchecked(1..) }})
                    }} else {{
                        None
                    }}
                }} else {{
                    None
                }}
            }}
            "#)
        }
        FPestExpr::Choice(items) => {
            let mut code = String::new();
            let mut quick_code = String::new();
            let mut error_code = String::from("    let mut errors = Vec::new();\n");
            for (i, item) in items.iter().enumerate() {
                let bid = ids.id(item);
                let idents = match contains_idents(item, has_whitespace) {
                    true => "idents",
                    false => "",
                };
                let cancel = if i == 0 { cancel1 } else { cancel2 } ;
                code.push_str(&format!("{cancel}    if let Some(input) = quick_parse_{bid}(input, {idents}) {{ return Ok(input); }}\n"));
                quick_code.push_str(&format!("{cancel}    if let Some(input) = quick_parse_{bid}(input, {idents}) {{ return Some(input); }}\n"));
                error_code.push_str(&format!("    errors.push(parse_{bid}(input, {idents}).unwrap_err());\n"));
            }

            format!(r#"
            // {hr_expr}
            pub fn parse_{id}<'i, 'b>(input: &'i [u8], {formatted_idents}) -> Result<&'i [u8], Error> {{
            {code}
            {error_code}
                {cancel2}
                Err(Error::new(ErrorKind::All(errors), unsafe{{std::str::from_utf8_unchecked(input)}}, "{id} {hr_expre}"))
            }}
            pub fn quick_parse_{id}<'i, 'b>(input: &'i [u8], {formatted_idents}) -> Option<&'i [u8]> {{
            {quick_code}
                {cancel2}
                None
            }}

            "#)
        }
        FPestExpr::Str(value) => {
            format!(r#"
            // {hr_expr}
            pub fn parse_{id}<'i>(input: &'i [u8]) -> Result<&'i [u8], Error> {{
                if input.starts_with(b{value:?}) {{
                    Ok(unsafe {{ input.get_unchecked({value:?}.len()..) }})
                }} else {{
                    Err(Error::new(ErrorKind::ExpectedValue({value:?}), unsafe{{std::str::from_utf8_unchecked(input)}}, "{id} {hr_expre}"))
                }}
            }}
            pub fn quick_parse_{id}<'i>(input: &'i [u8]) -> Option<&'i [u8]> {{
                if input.starts_with(b{value:?}) {{
                    Some(unsafe {{ input.get_unchecked({value:?}.len()..) }})
                }} else {{
                    None
                }}
            }}

            "#)
        }
        FPestExpr::Seq(items) => {
            let mut code = String::new();
            let mut note_for_next = String::new();
            let mut quick_code = String::new();
            for (i, item) in items.iter().enumerate() {
                let bid = ids.id(item);
                let idents = match contains_idents(item, has_whitespace) {
                    true => "idents",
                    false => "",
                };
                code.push_str(&format!("    input = parse_{bid}(input, {idents}).map_err(|e| e.with_trace(\"{id}-{i} {hr_expre}\"){note_for_next})?;\n"));
                quick_code.push_str(&format!("    input = quick_parse_{bid}(input, {idents})?;\n"));
                if has_whitespace {
                    code.push_str("    while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }\n");
                    quick_code.push_str("    while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }\n");
                }
                match item {
                    FPestExpr::Rep(rep, _) => {
                        let id = ids.id(rep);
                        let hr_rep = to_pest(rep);
                        let hr_repe = hr_rep.replace('\\', "\\\\").replace('"', "\\\"");
                        note_for_next = format!(".with_note(\"following sequence {id} {hr_repe}* which ended\")");
                    }
                    FPestExpr::Ident(i) if !CONDITIONS.iter().any(|(n,_)| n==i) => {
                        note_for_next = format!(".with_note(\"following {i} which ended\")"); // TODO: display if it contains a sequence
                    }
                    _ => note_for_next.clear(),
                }
            }

            format!(r#"
            // {hr_expr}
            pub fn parse_{id}<'i, 'b>(mut input: &'i [u8], {formatted_idents}) -> Result<&'i [u8], Error> {{
            {code}
                Ok(input)
            }}
            pub fn quick_parse_{id}<'i, 'b>(mut input: &'i [u8], {formatted_idents}) -> Option<&'i [u8]> {{
            {quick_code}
                Some(input)
            }}
            
            "#)
        }
        FPestExpr::Rep(expr, empty_accepted) => {
            if let FPestExpr::CharacterCondition(condition) = &**expr {
                if *empty_accepted {
                    return format!(r#"
                    // {hr_expr}
                    pub fn parse_{id}<'i, 'b>(mut input: &'i [u8], {formatted_idents}) -> Result<&'i [u8], Error> {{
                        let i = input.iter().position(|c| !({condition})).unwrap_or(input.len());
                        Ok(unsafe {{ input.get_unchecked(i..) }})
                    }}
                    pub fn quick_parse_{id}<'i, 'b>(mut input: &'i [u8], {formatted_idents}) -> Option<&'i [u8]> {{
                        let i = input.iter().position(|c| !({condition})).unwrap_or(input.len());
                        Some(unsafe {{ input.get_unchecked(i..) }})
                    }}
                        
                    "#);
                } else {
                    return format!(r#"
                    // {hr_expr}
                    pub fn parse_{id}<'i, 'b>(mut input: &'i [u8], {formatted_idents}) -> Result<&'i [u8], Error> {{
                        let i = input.iter().position(|c| !({condition})).unwrap_or(input.len());
                        if i == 0 {{
                            return Err(Error::new(ErrorKind::Expected("{condition}"), unsafe{{std::str::from_utf8_unchecked(input)}}, "{id} ({condition})+"));
                        }}
                        Ok(unsafe {{ input.get_unchecked(i..) }})
                    }}
                    pub fn quick_parse_{id}<'i, 'b>(mut input: &'i [u8], {formatted_idents}) -> Option<&'i [u8]> {{
                        let i = input.iter().position(|c| !({condition})).unwrap_or(input.len());
                        if i == 0 {{
                            return None;
                        }}
                        Some(unsafe {{ input.get_unchecked(i..) }})
                    }}
                    
                    "#);
                }
            }

            let expr_id = ids.id(expr);
            let idents = match contains_idents(expr, has_whitespace) {
                true => "idents",
                false => "",
            };

            let (whitespace, quick_whitespace) = match has_whitespace {
                true => ("while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }", "while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }"),
                false => ("", ""),
            };

            let (non_empty, quick_non_empty) = match empty_accepted {
                false => (format!("parse_{expr_id}(input, {idents})?;"), format!("quick_parse_{expr_id}(input, {idents})?;")),
                true => (String::new(), String::new()),
            };

            format!(r#"
            // {hr_expr}
            pub fn parse_{id}<'i, 'b>(mut input: &'i [u8], {formatted_idents}) -> Result<&'i [u8], Error> {{
                {non_empty}
                while let Ok(new_input) = parse_{expr_id}(input, {idents}) {{
                    input = new_input;
                    {whitespace}
                }}
                Ok(input)
            }}
            pub fn quick_parse_{id}<'i, 'b>(mut input: &'i [u8], {formatted_idents}) -> Option<&'i [u8]> {{
                {quick_non_empty}
                while let Some(new_input) = quick_parse_{expr_id}(input, {idents}) {{
                    input = new_input;
                    {quick_whitespace}
                }}
                Some(input)
            }}

            "#)
        }
        FPestExpr::Opt(expr) => {
            let expr_id = ids.id(expr);

            format!(r#"
            // {hr_expr}
            pub fn parse_{id}<'i, 'b>(input: &'i [u8], {formatted_idents}) -> Result<&'i [u8], Error> {{
                {cancel1}
                if let Ok(input) = parse_{expr_id}(input, {idents}) {{
                    Ok(input)
                }} else {{
                    {cancel2}
                    Ok(input)
                }}
            }}
            pub fn quick_parse_{id}<'i, 'b>(input: &'i [u8], {formatted_idents}) -> Option<&'i [u8]> {{
                {cancel1}
                if let Some(input) = quick_parse_{expr_id}(input, {idents}) {{
                    Some(input)
                }} else {{
                    {cancel2}
                    Some(input)
                }}
            }}
            "#)
        }
        FPestExpr::NegPred(expr) => {
            let expr_id = ids.id(expr);

            format!(r#"
            // {hr_expr}
            pub fn parse_{id}<'i, 'b>(input: &'i [u8], {formatted_idents}) -> Result<&'i [u8], Error> {{
                {cancel1}
                if parse_{expr_id}(input, {idents}).is_err() {{
                    {cancel2}
                    Ok(input)
                }} else {{
                    Err(Error::new(ErrorKind::NegPredFailed("{expr_id}"), unsafe{{std::str::from_utf8_unchecked(input)}}, "{id} {hr_expre}"))
                }}
            }}
            pub fn quick_parse_{id}<'i, 'b>(input: &'i [u8], {formatted_idents}) -> Option<&'i [u8]> {{
                {cancel1}
                if quick_parse_{expr_id}(input, {idents}).is_none() {{
                    {cancel2} // TODO: remove this
                    Some(input)
                }} else {{
                    None
                }}
            }}
            "#)
        }
        expr => todo!("code on {:?}", expr),
    }
}
