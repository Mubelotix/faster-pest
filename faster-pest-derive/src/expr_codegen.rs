use crate::*;

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

fn to_pest(expr: &OptimizedExpr) -> String {
    match expr {
        OptimizedExpr::Str(s) => format!("{s:?}"),
        OptimizedExpr::Insens(s) => format!("^{s:?}"),
        OptimizedExpr::Range(s, e) => format!("'{s}'..'{e}'"),
        OptimizedExpr::Ident(i) => i.to_owned(),
        OptimizedExpr::PeekSlice(_, _) => todo!(),
        OptimizedExpr::PosPred(e) => format!("&{}", to_pest(e)),
        OptimizedExpr::NegPred(e) => format!("!{}", to_pest(e)),
        OptimizedExpr::Seq(f, s) if matches!(s.as_ref(), OptimizedExpr::Rep(s) if f == s) => format!("{}+", to_pest(f)),
        OptimizedExpr::Seq(f, s) => {
            // TODO: This breaks ()+ detection
            /*let mut choices = Vec::new();
            list_seq(expr, &mut choices);
            format!("({})", choices.iter().map(|c| to_pest(c)).collect::<Vec<_>>().join(" ~ "))*/
            format!("({} ~ {})", to_pest(f), to_pest(s))
        },
        OptimizedExpr::Choice(_, _) => {
            let mut choices = Vec::new();
            list_choices(expr, &mut choices);
            format!("({})", choices.iter().map(|c| to_pest(c)).collect::<Vec<_>>().join(" | "))
        },
        OptimizedExpr::Opt(e) => format!("{}?", to_pest(e)),
        OptimizedExpr::Rep(e) => format!("{}*", to_pest(e)),
        OptimizedExpr::Skip(_) => todo!(),
        OptimizedExpr::Push(_) => todo!(),
        OptimizedExpr::RestoreOnErr(_) => todo!(),
    }
}

pub fn code(expr: &OptimizedExpr, ids: &mut IdRegistry, has_whitespace: bool) -> String {
    let id = ids.id(expr);
    let formatted_idents = match contains_idents(expr, has_whitespace) {
        true => "idents: &'b mut Vec<Ident<'i>>",
        false => "",
    };
    let (cancel1, cancel2, idents) = match contains_idents(expr, has_whitespace) {
        true => ("let idents_len = idents.len();", "idents.truncate(idents_len);", "idents"),
        false => ("", "", ""),
    };
    let hr_expr = to_pest(expr);
    let hr_expre = hr_expr.replace('\\', "\\\\").replace('\"', "\\\"");
    match expr {
        OptimizedExpr::Ident(ident) => {
            match ident.as_str() {
                "EOI" => {
                    format!(r#"
                    fn parse_{id}<'i>(input: &'i str) -> Result<&'i str, Error> {{
                        if input.is_empty() {{
                            Ok(input)
                        }} else {{
                            Err(Error::new(ErrorKind::Expected("EOI"), input, "EOI"))
                        }}
                    }}
                    fn quick_parse_{id}<'i>(input: &'i str) -> Option<&'i str> {{
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
                    fn parse_{id}<'i>(input: &'i str) -> Result<&'i str, Error> {{
                        Ok(input)
                    }}
                    fn quick_parse_{id}<'i>(input: &'i str) -> Option<&'i str> {{
                        Some(input)
                    }}

                    "#)
                }
                "NEWLINE" => {
                    format!(r#"
                    fn parse_{id}<'i>(input: &'i str) -> Result<&'i str, Error> {{
                        if input.starts_with("\r\n") {{
                            Ok(unsafe {{ input.get_unchecked(2..) }})
                        }} else if input.starts_with("\n") || input.starts_with("\r") {{
                            Ok(unsafe {{ input.get_unchecked(1..) }})
                        }} else {{
                            Err(Error::new(ErrorKind::Expected("newline"), input, "NEWLINE"))
                        }}
                    }}
                    fn quick_parse_{id}<'i>(input: &'i str) -> Option<&'i str> {{
                        if input.starts_with("\r\n") {{
                            Some(unsafe {{ input.get_unchecked(2..) }})
                        }} else if input.starts_with("\n") || input.starts_with("\r") {{
                            Some(unsafe {{ input.get_unchecked(1..) }})
                        }} else {{
                            None
                        }}
                    }}

                    "#)
                }
                other => if let Some((_, c)) = CONDITIONS.iter().find(|(n,_)| n == &other) {
                    format!(r#"
                    // {other}
                    fn parse_{id}<'i>(input: &'i str) -> Result<&'i str, Error> {{
                        if let Some(c) = input.as_bytes().first() {{
                            if {c} {{
                                Ok(unsafe {{ input.get_unchecked(1..) }})
                            }} else {{
                                Err(Error::new(ErrorKind::Expected("ASCII digit"), input, "{other}"))
                            }}
                        }} else {{
                            Err(Error::new(ErrorKind::Expected("ASCII digit"), input, "{other}"))
                        }}
                    }}
                    fn quick_parse_{id}<'i>(input: &'i str) -> Option<&'i str> {{
                        if let Some(c) = input.as_bytes().first() {{
                            if {c} {{
                                Some(unsafe {{ input.get_unchecked(1..) }})
                            }} else {{
                                None
                            }}
                        }} else {{
                            None
                        }}
                    }}
                    "#)
                } else {String::new()}
            }
        }
        OptimizedExpr::Choice(_, _) => {
            let mut choices = Vec::new();
            list_choices(expr, &mut choices);

            // If all choices are one character literals or character selectors, group them together
            let mut simple_conditions = Vec::new();
            for choice in &choices {
                match choice {
                    OptimizedExpr::Str(s) if s.len() == 1 => simple_conditions.push(format!("c == &b'{s}'")),
                    OptimizedExpr::Ident(i) => if let Some((_, c)) = CONDITIONS.iter().find(|(n,_)| n == i) {
                        simple_conditions.push(c.to_string());
                    }
                    _ => break,
                }
            }
            if simple_conditions.len() == choices.len() {
                let condition = simple_conditions.join(" || ");
                return format!(r#"
                // {condition}
                fn parse_{id}<'i>(input: &'i str) -> Result<&'i str, Error> {{
                    let b = input.as_bytes();
                    if !b.is_empty() {{
                        let c = unsafe {{ b.get_unchecked(0) }};
                        if {condition} {{
                            Ok(unsafe {{ input.get_unchecked(1..) }})
                        }} else {{
                            Err(Error::new(ErrorKind::Expected("ASCII digit"), input, "{id} {condition}"))
                        }}
                    }} else {{
                        Err(Error::new(ErrorKind::Expected("ASCII digit"), input, "{id} {condition}"))
                    }}
                }}
                fn quick_parse_{id}<'i>(input: &'i str) -> Option<&'i str> {{
                    let b = input.as_bytes();
                    if !b.is_empty() {{
                        let c = unsafe {{ b.get_unchecked(0) }};
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

            let mut code = String::new();
            let mut quick_code = String::new();
            let mut error_code = String::from("    let mut errors = Vec::new();\n");
            for (i, choice) in choices.iter().enumerate() {
                let bid = ids.id(choice);
                let idents = match contains_idents(choice, has_whitespace) {
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
            fn parse_{id}<'i, 'b>(input: &'i str, {formatted_idents}) -> Result<&'i str, Error> {{
            {code}
            {error_code}
                {cancel2}
                Err(Error::new(ErrorKind::All(errors), input, "{id} {hr_expre}"))
            }}
            fn quick_parse_{id}<'i, 'b>(input: &'i str, {formatted_idents}) -> Option<&'i str> {{
            {quick_code}
                {cancel2}
                None
            }}

            "#)
        }
        OptimizedExpr::Str(value) => {
            format!(r#"
            // {hr_expr}
            fn parse_{id}<'i>(input: &'i str) -> Result<&'i str, Error> {{
                if input.starts_with({value:?}) {{
                    Ok(unsafe {{ input.get_unchecked({value:?}.len()..) }})
                }} else {{
                    Err(Error::new(ErrorKind::ExpectedValue({value:?}), input, "{id} {hr_expre}"))
                }}
            }}
            fn quick_parse_{id}<'i>(input: &'i str) -> Option<&'i str> {{
                if input.starts_with({value:?}) {{
                    Some(unsafe {{ input.get_unchecked({value:?}.len()..) }})
                }} else {{
                    None
                }}
            }}

            "#)
        }
        OptimizedExpr::Seq(_, _) => {
            let mut seq = Vec::new();
            list_seq(expr, &mut seq);

            let mut code = String::new();
            let mut note_for_next = String::new();
            let mut quick_code = String::new();
            for (i, seq) in seq.iter().enumerate() {
                let bid = ids.id(seq);
                let idents = match contains_idents(seq, has_whitespace) {
                    true => "idents",
                    false => "",
                };
                code.push_str(&format!("    input = parse_{bid}(input, {idents}).map_err(|e| e.with_trace(\"{id}-{i} {hr_expre}\"){note_for_next})?;\n"));
                quick_code.push_str(&format!("    input = quick_parse_{bid}(input, {idents})?;\n"));
                if has_whitespace {
                    code.push_str("    while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }\n");
                    quick_code.push_str("    while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }\n");
                }
                match seq {
                    OptimizedExpr::Rep(rep) => {
                        let id = ids.id(rep);
                        let hr_rep = to_pest(rep);
                        let hr_repe = hr_rep.replace('\\', "\\\\").replace('"', "\\\"");
                        note_for_next = format!(".with_note(\"following sequence {id} {hr_repe}* which ended\")");
                    }
                    OptimizedExpr::Ident(i) if !CONDITIONS.iter().any(|(n,_)| n==i) => {
                        note_for_next = format!(".with_note(\"following {i} which ended\")"); // TODO: display if it contains a sequence
                    }
                    _ => note_for_next.clear(),
                }
            }


            format!(r#"
            // {hr_expr}
            fn parse_{id}<'i, 'b>(mut input: &'i str, {formatted_idents}) -> Result<&'i str, Error> {{
            {code}
                Ok(input)
            }}
            fn quick_parse_{id}<'i, 'b>(mut input: &'i str, {formatted_idents}) -> Option<&'i str> {{
            {quick_code}
                Some(input)
            }}

            "#)
        }
        OptimizedExpr::Rep(expr) => {
            // If we loop over a character selector, optimize this away
            if matches!(expr.as_ref(), OptimizedExpr::Choice(_, _)) {
                let mut choices = Vec::new();
                list_choices(expr, &mut choices);
                let mut simple_conditions = Vec::new();
                for choice in &choices {
                    match choice {
                        OptimizedExpr::Str(s) if s.len() == 1 => simple_conditions.push(format!("c == &b'{s}'")),
                        OptimizedExpr::Ident(i) => if let Some((_, c)) = CONDITIONS.iter().find(|(n,_)| n == i) {
                            simple_conditions.push(c.to_string());
                        }
                        _ => break,
                    }
                }
                if simple_conditions.len() == choices.len() {
                    let condition = simple_conditions.join(" || ");
                    return format!(r#"
                    // {hr_expr}
                    fn parse_{id}<'i, 'b>(mut input: &'i str, {formatted_idents}) -> Result<&'i str, Error> {{
                        let i = input.as_bytes().iter().position(|c| !({condition})).unwrap_or(0);
                        Ok(unsafe {{ input.get_unchecked(i..) }})
                    }}
                    fn quick_parse_{id}<'i, 'b>(mut input: &'i str, {formatted_idents}) -> Option<&'i str> {{
                        let i = input.as_bytes().iter().position(|c| !({condition})).unwrap_or(0);
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

            format!(r#"
            // {hr_expr}
            fn parse_{id}<'i, 'b>(mut input: &'i str, {formatted_idents}) -> Result<&'i str, Error> {{
                while let Ok(new_input) = parse_{expr_id}(input, {idents}) {{
                    input = new_input;
                    {whitespace}
                }}
                Ok(input)
            }}
            fn quick_parse_{id}<'i, 'b>(mut input: &'i str, {formatted_idents}) -> Option<&'i str> {{
                while let Some(new_input) = quick_parse_{expr_id}(input, {idents}) {{
                    input = new_input;
                    {quick_whitespace}
                }}
                Some(input)
            }}

            "#)
        }
        OptimizedExpr::Opt(expr) => {
            let expr_id = ids.id(expr);

            format!(r#"
            // {hr_expr}
            fn parse_{id}<'i, 'b>(input: &'i str, {formatted_idents}) -> Result<&'i str, Error> {{
                {cancel1}
                if let Ok(input) = parse_{expr_id}(input, {idents}) {{
                    Ok(input)
                }} else {{
                    {cancel2}
                    Ok(input)
                }}
            }}
            fn quick_parse_{id}<'i, 'b>(input: &'i str, {formatted_idents}) -> Option<&'i str> {{
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
        OptimizedExpr::NegPred(expr) => {
            let expr_id = ids.id(expr);

            format!(r#"
            // {hr_expr}
            fn parse_{id}<'i, 'b>(input: &'i str, {formatted_idents}) -> Result<&'i str, Error> {{
                {cancel1}
                if parse_{expr_id}(input, {idents}).is_err() {{
                    {cancel2}
                    Ok(input)
                }} else {{
                    Err(Error::new(ErrorKind::NegPredFailed("{expr_id}"), input, "{id} {hr_expre}"))
                }}
            }}
            fn quick_parse_{id}<'i, 'b>(input: &'i str, {formatted_idents}) -> Option<&'i str> {{
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
