#[derive(Debug)]
enum ErrorKind {
    ExpectedValue(&'static str),
    Expected(&'static str),
    Both(Box<Error>, Box<Error>)
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

    fn add_trace(&mut self, trace: impl Into<String>) {
        self.trace.push(trace.into());
    }
}
