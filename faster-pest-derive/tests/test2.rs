const RED: &str = "\x1b[31;1m";
const NORMAL: &str = "\x1b[0m";
const BLUE: &str = "\x1b[34;1m";

#[derive(Debug)]
enum ErrorKind {
    ExpectedValue(&'static str),
    Expected(&'static str),
    All(Vec<Error>)
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorKind::ExpectedValue(expected) => write!(f, "Expected value: {}", expected),
            ErrorKind::Expected(expected) => write!(f, "Expected: {}", expected),
            ErrorKind::All(errors) => write!(f, "All {} accepted patterns fail to match", errors.len()),
        }
    }
}

#[derive(Debug)]
struct Error {
    kind: ErrorKind,
    remaining_bytes: usize,
    trace: Vec<String>,
}

impl Error {
    fn new(kind: ErrorKind, input: &str, root: impl Into<String>) -> Error {
        Error {
            kind,
            remaining_bytes: input.len(),
            trace: vec![root.into()],
        }
    }

    fn with_trace(mut self, trace: impl Into<String>) -> Self {
        self.trace.push(trace.into());
        self
    }

    fn print(&self, input: &str) {
        if self.remaining_bytes > input.len() {
            panic!("Error::print: remaining_bytes is greater than input length");
        }
        let position = input.len() - self.remaining_bytes;

        let line_start = input[..position].rfind('\n').map(|i| i + 1).unwrap_or(0);
        let line_end = input[position..].find('\n').map(|i| i + position).unwrap_or(input.len());
        let line_number = input[..position].matches('\n').count() + 1;
        let position_in_utf8_line = input[line_start..position].chars().count();

        println!("{RED}error{NORMAL}: {}", self.kind);
        println!("{BLUE} -->{NORMAL} {}:{}:{}", self.trace[0], line_number, position - line_start + 1);
        println!("   {BLUE}|{NORMAL}");
        println!("{:>3}{BLUE}|{NORMAL} {}", line_number, &input[line_start..line_end]);
        println!("   {BLUE}|{NORMAL} {}{RED}^{NORMAL}", " ".repeat(position_in_utf8_line));
        println!("   {BLUE}= {NORMAL}note: {}", self.trace.join(", "));
    }
}

    

    

    type Res<'i> = Result<&'i str, Error>;
    #[derive(Debug)]
pub enum Ident<'i> {
    Linecontent(&'i str),
    Line(&'i str),
    Msgid(&'i str),
    Msgstr(&'i str),
    Entry(&'i str),
    File(&'i str),
}


                fn parse_linecontent<'i, 'b>(input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Res<'i> {
                    let idents_len = idents.len();
                    idents.push(Ident::Linecontent(""));
                    let new_input = match parse_anon_0(input, idents) {
                        Ok(input) => input,
                        Err(e) => {
                            idents.truncate(idents_len);
                            return Err(e);
                        }
                    };
                    let new_ident = &input[..input.len() - new_input.len()];
                    idents[idents_len] = Ident::Linecontent(new_ident);
                    Ok(new_input)
                }

                fn quick_parse_linecontent<'i, 'b>(input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Option<&'i str> {
                    let idents_len = idents.len();
                    idents.push(Ident::Linecontent(""));
                    let new_input = match quick_parse_anon_0(input, idents) {
                        Some(input) => input,
                        None => {
                            idents.truncate(idents_len);
                            return None;
                        }
                    };
                    let new_ident = &input[..input.len() - new_input.len()];
                    idents[idents_len] = Ident::Linecontent(new_ident);
                    Some(new_input)
                }
                
                fn parse_line<'i, 'b>(input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Res<'i> {
                    let idents_len = idents.len();
                    idents.push(Ident::Line(""));
                    let new_input = match parse_anon_1(input, idents) {
                        Ok(input) => input,
                        Err(e) => {
                            idents.truncate(idents_len);
                            return Err(e);
                        }
                    };
                    let new_ident = &input[..input.len() - new_input.len()];
                    idents[idents_len] = Ident::Line(new_ident);
                    Ok(new_input)
                }

                fn quick_parse_line<'i, 'b>(input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Option<&'i str> {
                    let idents_len = idents.len();
                    idents.push(Ident::Line(""));
                    let new_input = match quick_parse_anon_1(input, idents) {
                        Some(input) => input,
                        None => {
                            idents.truncate(idents_len);
                            return None;
                        }
                    };
                    let new_ident = &input[..input.len() - new_input.len()];
                    idents[idents_len] = Ident::Line(new_ident);
                    Some(new_input)
                }
                
                fn parse_msgid<'i, 'b>(input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Res<'i> {
                    let idents_len = idents.len();
                    idents.push(Ident::Msgid(""));
                    let new_input = match parse_anon_2(input, idents) {
                        Ok(input) => input,
                        Err(e) => {
                            idents.truncate(idents_len);
                            return Err(e);
                        }
                    };
                    let new_ident = &input[..input.len() - new_input.len()];
                    idents[idents_len] = Ident::Msgid(new_ident);
                    Ok(new_input)
                }

                fn quick_parse_msgid<'i, 'b>(input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Option<&'i str> {
                    let idents_len = idents.len();
                    idents.push(Ident::Msgid(""));
                    let new_input = match quick_parse_anon_2(input, idents) {
                        Some(input) => input,
                        None => {
                            idents.truncate(idents_len);
                            return None;
                        }
                    };
                    let new_ident = &input[..input.len() - new_input.len()];
                    idents[idents_len] = Ident::Msgid(new_ident);
                    Some(new_input)
                }
                
                fn parse_msgstr<'i, 'b>(input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Res<'i> {
                    let idents_len = idents.len();
                    idents.push(Ident::Msgstr(""));
                    let new_input = match parse_anon_3(input, idents) {
                        Ok(input) => input,
                        Err(e) => {
                            idents.truncate(idents_len);
                            return Err(e);
                        }
                    };
                    let new_ident = &input[..input.len() - new_input.len()];
                    idents[idents_len] = Ident::Msgstr(new_ident);
                    Ok(new_input)
                }

                fn quick_parse_msgstr<'i, 'b>(input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Option<&'i str> {
                    let idents_len = idents.len();
                    idents.push(Ident::Msgstr(""));
                    let new_input = match quick_parse_anon_3(input, idents) {
                        Some(input) => input,
                        None => {
                            idents.truncate(idents_len);
                            return None;
                        }
                    };
                    let new_ident = &input[..input.len() - new_input.len()];
                    idents[idents_len] = Ident::Msgstr(new_ident);
                    Some(new_input)
                }
                
                fn parse_entry<'i, 'b>(input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Res<'i> {
                    let idents_len = idents.len();
                    idents.push(Ident::Entry(""));
                    let new_input = match parse_anon_4(input, idents) {
                        Ok(input) => input,
                        Err(e) => {
                            idents.truncate(idents_len);
                            return Err(e);
                        }
                    };
                    let new_ident = &input[..input.len() - new_input.len()];
                    idents[idents_len] = Ident::Entry(new_ident);
                    Ok(new_input)
                }

                fn quick_parse_entry<'i, 'b>(input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Option<&'i str> {
                    let idents_len = idents.len();
                    idents.push(Ident::Entry(""));
                    let new_input = match quick_parse_anon_4(input, idents) {
                        Some(input) => input,
                        None => {
                            idents.truncate(idents_len);
                            return None;
                        }
                    };
                    let new_ident = &input[..input.len() - new_input.len()];
                    idents[idents_len] = Ident::Entry(new_ident);
                    Some(new_input)
                }
                
                fn parse_file<'i, 'b>(input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Res<'i> {
                    let idents_len = idents.len();
                    idents.push(Ident::File(""));
                    let new_input = match parse_anon_5(input, idents) {
                        Ok(input) => input,
                        Err(e) => {
                            idents.truncate(idents_len);
                            return Err(e);
                        }
                    };
                    let new_ident = &input[..input.len() - new_input.len()];
                    idents[idents_len] = Ident::File(new_ident);
                    Ok(new_input)
                }

                fn quick_parse_file<'i, 'b>(input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Option<&'i str> {
                    let idents_len = idents.len();
                    idents.push(Ident::File(""));
                    let new_input = match quick_parse_anon_5(input, idents) {
                        Some(input) => input,
                        None => {
                            idents.truncate(idents_len);
                            return None;
                        }
                    };
                    let new_ident = &input[..input.len() - new_input.len()];
                    idents[idents_len] = Ident::File(new_ident);
                    Some(new_input)
                }
                
                fn parse_WHITESPACE<'i, 'b>(input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Res<'i> {
                    parse_anon_6(input, )
                }

                fn quick_parse_WHITESPACE<'i, 'b>(input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Option<&'i str> {
                    quick_parse_anon_6(input, )
                }
                
fn parse_ASCII_ALPHANUMERIC<'i>(input: &'i str) -> Res<'i> {
    if let Some(first) = input.chars().next() {
        if first.is_ascii_alphanumeric() {
            Ok(&input[1..])
        } else {
            Err(Error::new(ErrorKind::Expected("ASCII alphanumeric"), input, "ASCII_ALPHANUMERIC"))
        }
    } else {
        Err(Error::new(ErrorKind::Expected("ASCII alphanumeric"), input, "ASCII_ALPHANUMERIC"))
    }
}

fn quick_parse_ASCII_ALPHANUMERIC<'i>(input: &'i str) -> Option<&'i str> {
    if let Some(first) = input.chars().next() {
        if first.is_ascii_alphanumeric() {
            Some(&input[1..])
        } else {
            None
        }
    } else {
        None
    }
}


fn parse_EOI<'i>(input: &'i str) -> Res<'i> {
    if input.is_empty() {
        Ok(input)
    } else {
        Err(Error::new(ErrorKind::Expected("EOI"), input, "EOI"))
    }
}

fn quick_parse_EOI<'i>(input: &'i str) -> Option<&'i str> {
    if input.is_empty() {
        Some(input)
    } else {
        None
    }
}


fn parse_NEWLINE<'i>(input: &'i str) -> Res<'i> {
    if input.starts_with("\r\n") {
        Ok(&input[2..])
    } else if input.starts_with("\n") || input.starts_with("\r") {
        Ok(&input[1..])
    } else {
        Err(Error::new(ErrorKind::Expected("newline"), input, "NEWLINE"))
    }
}

fn quick_parse_NEWLINE<'i>(input: &'i str) -> Option<&'i str> {
    if input.starts_with("\r\n") {
        Some(&input[2..])
    } else if input.starts_with("\n") || input.starts_with("\r") {
        Some(&input[1..])
    } else {
        None
    }
}


fn parse_NEWLINE<'i>(input: &'i str) -> Res<'i> {
    if input.starts_with("\r\n") {
        Ok(&input[2..])
    } else if input.starts_with("\n") || input.starts_with("\r") {
        Ok(&input[1..])
    } else {
        Err(Error::new(ErrorKind::Expected("newline"), input, "NEWLINE"))
    }
}

fn quick_parse_NEWLINE<'i>(input: &'i str) -> Option<&'i str> {
    if input.starts_with("\r\n") {
        Some(&input[2..])
    } else if input.starts_with("\n") || input.starts_with("\r") {
        Some(&input[1..])
    } else {
        None
    }
}

 // TODO
                       fn parse_SOI<'i>(input: &'i str) -> Res<'i> {
                           Ok(input)
                       }

                       fn quick_parse_SOI<'i>(input: &'i str) -> Option<&'i str> {
                           Some(input)
                       }

                       
fn parse_anon_0<'i, 'b>(mut input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Res<'i> {
    while let Ok(new_input) = parse_ASCII_ALPHANUMERIC(input, ) {
        input = new_input;
        while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
    }
    Ok(input)
}

fn quick_parse_anon_0<'i, 'b>(mut input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Option<&'i str> {
    while let Some(new_input) = quick_parse_ASCII_ALPHANUMERIC(input, ) {
        input = new_input;
        while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    }
    Some(input)
}


fn parse_anon_1<'i, 'b>(mut input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Res<'i> {
    input = parse_anon_13(input, ).map_err(|e| e.with_trace("sequence anon_1 arm 0"))?;
input = parse_linecontent(input, idents).map_err(|e| e.with_trace("sequence anon_1 arm 1"))?;
input = parse_anon_13(input, ).map_err(|e| e.with_trace("sequence anon_1 arm 2"))?;
input = parse_NEWLINE(input, ).map_err(|e| e.with_trace("sequence anon_1 arm 3"))?;

    Ok(input)
}

fn quick_parse_anon_1<'i, 'b>(mut input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Option<&'i str> {
    input = quick_parse_anon_13(input, )?;
input = quick_parse_linecontent(input, idents)?;
input = quick_parse_anon_13(input, )?;
input = quick_parse_NEWLINE(input, )?;

    Some(input)
}


fn parse_anon_10<'i, 'b>(mut input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Res<'i> {
    while let Ok(new_input) = parse_line(input, idents) {
        input = new_input;
        while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
    }
    Ok(input)
}

fn quick_parse_anon_10<'i, 'b>(mut input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Option<&'i str> {
    while let Some(new_input) = quick_parse_line(input, idents) {
        input = new_input;
        while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    }
    Some(input)
}


fn parse_anon_11<'i>(input: &'i str) -> Res<'i> {
    if input.starts_with("msgstr ") {
        Ok(&input["msgstr ".len()..])
    } else {
        Err(Error::new(ErrorKind::ExpectedValue("msgstr "), input, "anon_11"))
    }
}

fn quick_parse_anon_11<'i>(input: &'i str) -> Option<&'i str> {
    if input.starts_with("msgstr ") {
        Some(&input["msgstr ".len()..])
    } else {
        None
    }
}


fn parse_anon_12<'i>(input: &'i str) -> Res<'i> {
    if input.starts_with("msgid ") {
        Ok(&input["msgid ".len()..])
    } else {
        Err(Error::new(ErrorKind::ExpectedValue("msgid "), input, "anon_12"))
    }
}

fn quick_parse_anon_12<'i>(input: &'i str) -> Option<&'i str> {
    if input.starts_with("msgid ") {
        Some(&input["msgid ".len()..])
    } else {
        None
    }
}


fn parse_anon_13<'i>(input: &'i str) -> Res<'i> {
    if input.starts_with("\"") {
        Ok(&input["\"".len()..])
    } else {
        Err(Error::new(ErrorKind::ExpectedValue("\""), input, "anon_13"))
    }
}

fn quick_parse_anon_13<'i>(input: &'i str) -> Option<&'i str> {
    if input.starts_with("\"") {
        Some(&input["\"".len()..])
    } else {
        None
    }
}


fn parse_anon_2<'i, 'b>(mut input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Res<'i> {
    input = parse_anon_12(input, ).map_err(|e| e.with_trace("sequence anon_2 arm 0"))?;
while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
input = parse_line(input, idents).map_err(|e| e.with_trace("sequence anon_2 arm 1"))?;
while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
input = parse_anon_10(input, idents).map_err(|e| e.with_trace("sequence anon_2 arm 2"))?;
while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }

    Ok(input)
}

fn quick_parse_anon_2<'i, 'b>(mut input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Option<&'i str> {
    input = quick_parse_anon_12(input, )?;
while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
input = quick_parse_line(input, idents)?;
while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
input = quick_parse_anon_10(input, idents)?;
while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }

    Some(input)
}


fn parse_anon_3<'i, 'b>(mut input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Res<'i> {
    input = parse_anon_11(input, ).map_err(|e| e.with_trace("sequence anon_3 arm 0"))?;
while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
input = parse_line(input, idents).map_err(|e| e.with_trace("sequence anon_3 arm 1"))?;
while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
input = parse_anon_10(input, idents).map_err(|e| e.with_trace("sequence anon_3 arm 2"))?;
while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }

    Ok(input)
}

fn quick_parse_anon_3<'i, 'b>(mut input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Option<&'i str> {
    input = quick_parse_anon_11(input, )?;
while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
input = quick_parse_line(input, idents)?;
while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
input = quick_parse_anon_10(input, idents)?;
while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }

    Some(input)
}


fn parse_anon_4<'i, 'b>(mut input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Res<'i> {
    input = parse_msgid(input, idents).map_err(|e| e.with_trace("sequence anon_4 arm 0"))?;
while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
input = parse_msgstr(input, idents).map_err(|e| e.with_trace("sequence anon_4 arm 1"))?;
while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }

    Ok(input)
}

fn quick_parse_anon_4<'i, 'b>(mut input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Option<&'i str> {
    input = quick_parse_msgid(input, idents)?;
while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
input = quick_parse_msgstr(input, idents)?;
while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }

    Some(input)
}


fn parse_anon_5<'i, 'b>(mut input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Res<'i> {
    input = parse_SOI(input, ).map_err(|e| e.with_trace("sequence anon_5 arm 0"))?;
while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
input = parse_entry(input, idents).map_err(|e| e.with_trace("sequence anon_5 arm 1"))?;
while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
input = parse_anon_9(input, idents).map_err(|e| e.with_trace("sequence anon_5 arm 2"))?;
while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
input = parse_anon_7(input, idents).map_err(|e| e.with_trace("sequence anon_5 arm 3"))?;
while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
input = parse_EOI(input, ).map_err(|e| e.with_trace("sequence anon_5 arm 4"))?;
while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }

    Ok(input)
}

fn quick_parse_anon_5<'i, 'b>(mut input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Option<&'i str> {
    input = quick_parse_SOI(input, )?;
while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
input = quick_parse_entry(input, idents)?;
while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
input = quick_parse_anon_9(input, idents)?;
while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
input = quick_parse_anon_7(input, idents)?;
while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
input = quick_parse_EOI(input, )?;
while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }

    Some(input)
}


fn parse_anon_6<'i>(input: &'i str) -> Res<'i> {
    if input.starts_with(" ") {
        Ok(&input[" ".len()..])
    } else {
        Err(Error::new(ErrorKind::ExpectedValue(" "), input, "anon_6"))
    }
}

fn quick_parse_anon_6<'i>(input: &'i str) -> Option<&'i str> {
    if input.starts_with(" ") {
        Some(&input[" ".len()..])
    } else {
        None
    }
}


fn parse_anon_7<'i, 'b>(mut input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Res<'i> {
    while let Ok(new_input) = parse_anon_8(input, idents) {
        input = new_input;
        while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
    }
    Ok(input)
}

fn quick_parse_anon_7<'i, 'b>(mut input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Option<&'i str> {
    while let Some(new_input) = quick_parse_anon_8(input, idents) {
        input = new_input;
        while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    }
    Some(input)
}


fn parse_anon_8<'i, 'b>(mut input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Res<'i> {
    input = parse_entry(input, idents).map_err(|e| e.with_trace("sequence anon_8 arm 0"))?;
while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
input = parse_anon_9(input, idents).map_err(|e| e.with_trace("sequence anon_8 arm 1"))?;
while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }

    Ok(input)
}

fn quick_parse_anon_8<'i, 'b>(mut input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Option<&'i str> {
    input = quick_parse_entry(input, idents)?;
while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
input = quick_parse_anon_9(input, idents)?;
while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }

    Some(input)
}


fn parse_anon_9<'i, 'b>(mut input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Res<'i> {
    while let Ok(new_input) = parse_NEWLINE(input, ) {
        input = new_input;
        while let Ok(new_input) = parse_WHITESPACE(input, idents) { input = new_input }
    }
    Ok(input)
}

fn quick_parse_anon_9<'i, 'b>(mut input: &'i str, idents: &'b mut Vec<Ident<'i>>) -> Option<&'i str> {
    while let Some(new_input) = quick_parse_NEWLINE(input, ) {
        input = new_input;
        while let Some(new_input) = quick_parse_WHITESPACE(input, idents) { input = new_input }
    }
    Some(input)
}



#[test]
fn test() {
    let input = r#"msgid "test"
    "test2"
msgstr   "test"

msgid "apricot"
msgstr ""
"test"
"#;
    let mut idents = Vec::new();
    match parse_file(input, &mut idents) {
        Ok(_) => {}
        Err(e) => e.print(input)
    }
    dbg!(idents);
}
