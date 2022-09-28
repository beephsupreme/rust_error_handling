use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::{stdin, Read};

#[derive(Debug)]
#[non_exhaustive]
enum MyError {
    ParseError,
    ReadError,
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        use MyError::*;
        match self {
            ParseError => write!(f, "PARSE ERROR"),
            ReadError => write!(f, "READ ERROR"),
        }
    }
}

impl Error for MyError {}

fn main() {
    let n = match get_int() {
        Ok(n) => n,
        Err(e) => panic!("{}: {}", e, "Could not get n"),
    };
    println!("{}", n);
}

fn get_int() -> Result<usize, MyError> {
    let n = {
        let mut buffer = String::new();
        stdin().lock().read_to_string(&mut buffer).unwrap();
        let mut tokens = buffer.split_ascii_whitespace();
        let n = tokens
            .next()
            .unwrap()
            .parse::<usize>()
            .map_err(|_| MyError::ParseError);
        n
    };
    n
}
