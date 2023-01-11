use crate::*;

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
    match expr {
        OptimizedExpr::Ident(ident) => {
            match ident.as_str() {
                "ASCII_DIGIT" => {
                    format!(r#"
                    fn parse_{id}<'i>(input: &'i str) -> Result<&'i str, Error> {{
                        if let Some(first) = input.chars().next() {{
                            if first.is_ascii_digit() {{
                                Ok(&input[1..])
                            }} else {{
                                Err(Error::new(ErrorKind::Expected("ASCII digit"), input, "ASCII_DIGIT"))
                            }}
                        }} else {{
                            Err(Error::new(ErrorKind::Expected("ASCII digit"), input, "ASCII_DIGIT"))
                        }}
                    }}

                    fn quick_parse_{id}<'i>(input: &'i str) -> Option<&'i str> {{
                        if let Some(first) = input.chars().next() {{
                            if first.is_ascii_digit() {{
                                Some(&input[1..])
                            }} else {{
                                None
                            }}
                        }} else {{
                            None
                        }}
                    }}

                    "#)
                }
                "ASCII_ALPHANUMERIC" => {
                    format!(r#"
                    fn parse_{id}<'i>(input: &'i str) -> Result<&'i str, Error> {{
                        if let Some(first) = input.chars().next() {{
                            if first.is_ascii_alphanumeric() {{
                                Ok(&input[1..])
                            }} else {{
                                Err(Error::new(ErrorKind::Expected("ASCII alphanumeric"), input, "ASCII_ALPHANUMERIC"))
                            }}
                        }} else {{
                            Err(Error::new(ErrorKind::Expected("ASCII alphanumeric"), input, "ASCII_ALPHANUMERIC"))
                        }}
                    }}

                    fn quick_parse_{id}<'i>(input: &'i str) -> Option<&'i str> {{
                        if let Some(first) = input.chars().next() {{
                            if first.is_ascii_alphanumeric() {{
                                Some(&input[1..])
                            }} else {{
                                None
                            }}
                        }} else {{
                            None
                        }}
                    }}

                    "#)
                }
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
                            Ok(&input[2..])
                        }} else if input.starts_with("\n") || input.starts_with("\r") {{
                            Ok(&input[1..])
                        }} else {{
                            Err(Error::new(ErrorKind::Expected("newline"), input, "NEWLINE"))
                        }}
                    }}

                    fn quick_parse_{id}<'i>(input: &'i str) -> Option<&'i str> {{
                        if input.starts_with("\r\n") {{
                            Some(&input[2..])
                        }} else if input.starts_with("\n") || input.starts_with("\r") {{
                            Some(&input[1..])
                        }} else {{
                            None
                        }}
                    }}

                    "#)
                }
                _ => String::new()
            }
        }
        OptimizedExpr::Choice(_, _) => {
            let mut choices = Vec::new();
            list_choices(expr, &mut choices);

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
            fn parse_{id}<'i, 'b>(input: &'i str, {formatted_idents}) -> Result<&'i str, Error> {{
            {code}
            {error_code}
                {cancel2}
                Err(Error::new(ErrorKind::All(errors), input, "choice {id}"))
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
            fn parse_{id}<'i>(input: &'i str) -> Result<&'i str, Error> {{
                if input.starts_with({value:?}) {{
                    Ok(&input[{value:?}.len()..])
                }} else {{
                    Err(Error::new(ErrorKind::ExpectedValue({value:?}), input, "{id}"))
                }}
            }}

            fn quick_parse_{id}<'i>(input: &'i str) -> Option<&'i str> {{
                if input.starts_with({value:?}) {{
                    Some(&input[{value:?}.len()..])
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
            let mut quick_code = String::new();
            for (i, seq) in seq.iter().enumerate() {
                let bid = ids.id(seq);
                let idents = match contains_idents(seq, has_whitespace) {
                    true => "idents",
                    false => "",
                };
                code.push_str(&format!("    input = parse_{bid}(input, {idents}).map_err(|e| e.with_trace(\"sequence {id} arm {i}\"))?;\n"));
                quick_code.push_str(&format!("    input = quick_parse_{bid}(input, {idents})?;\n"));
                if has_whitespace {
                    code.push_str("    while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }\n");
                    quick_code.push_str("    while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }\n");
                }
            }


            format!(r#"
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
        expr => todo!("code on {:?}", expr),
    }
}
