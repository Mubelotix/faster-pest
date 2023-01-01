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
}

impl Error {
    fn new(kind: ErrorKind, input: &str) -> Error {
        Error {
            kind,
            remaining_bytes: input.len(),
        }
    }
}
