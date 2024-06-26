const RED: &str = "\x1b[31;1m";
const NORMAL: &str = "\x1b[0m";
const BLUE: &str = "\x1b[34;1m";
const BOLD: &str = "\x1b[1m";

#[derive(Debug)]
pub enum ErrorKind {
    ExpectedValue(&'static str),
    Expected(&'static str),
    NegPredFailed(&'static str),
    All(Vec<Error>)
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorKind::ExpectedValue(expected) => write!(f, "Expected value: {expected}"),
            ErrorKind::Expected(expected) => write!(f, "Expected: {expected}"),
            ErrorKind::NegPredFailed(not_expected) => write!(f, "Negated predicate failed: {not_expected}"),
            ErrorKind::All(errors) => write!(f, "All {} accepted patterns fail to match", errors.len()),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    remaining_bytes: usize,
    trace: Vec<String>,
    note: Option<String>,
}

impl Error {
    pub fn new(kind: ErrorKind, input: &str, root: impl Into<String>) -> Error {
        Error {
            kind,
            remaining_bytes: input.len(),
            trace: vec![root.into()],
            note: None,
        }
    }

    pub fn with_trace(mut self, trace: impl Into<String>) -> Self {
        self.trace.push(trace.into());
        self
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        if self.note.is_none() {
            self.note = Some(note.into());
        }
        self
    }

    pub fn print(&self, input: &str) {
        if self.remaining_bytes > input.len() {
            panic!("Error::print: remaining_bytes is greater than input length");
        }
        let position = input.len() - self.remaining_bytes;

        let line_start = input[..position].rfind('\n').map(|i| i + 1).unwrap_or(0);
        let line_end = input[position..].find('\n').map(|i| i + position).unwrap_or(input.len());
        let line_number = input[..position].matches('\n').count() + 1;
        let position_in_utf8_line = input[line_start..position].chars().count();

        println!("{RED}error{NORMAL}: {}", self.kind);
        println!("{BLUE}  -->{NORMAL} {}:{}:{}", self.trace[0], line_number, position - line_start + 1);
        println!("{BLUE}   |{NORMAL}");
        println!("{BLUE}{:<3}|{NORMAL} {}", line_number, &input[line_start..line_end]);
        println!("{BLUE}   |{NORMAL} {}{RED}^{NORMAL}", " ".repeat(position_in_utf8_line));
        if let Some(note) = &self.note {
            println!("   {BLUE}= {NORMAL}{BOLD}note{NORMAL}: {note}");
        }
        println!("   {BLUE}= {NORMAL}{BOLD}trace{NORMAL}: {}", self.trace.join(", "));
    }

    pub fn into_pest<Rule: pest::RuleType>(self, input: &str) -> pest::error::Error<Rule> {
        pest::error::Error::new_from_pos(
            pest::error::ErrorVariant::CustomError {
                message: format!("{}", self.kind),
            },
            pest::Position::new(input, input.len() - self.remaining_bytes).expect("Error::into_pest: invalid position"),
        )
    }
}
