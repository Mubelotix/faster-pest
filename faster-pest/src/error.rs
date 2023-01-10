const RED: &str = "\x1b[31m";
const NORMAL: &str = "\x1b[0m";
const BLUE: &str = "\x1b[34m";

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
