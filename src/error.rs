use thiserror::Error;
use nom::error::ErrorKind;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("SyntaxError: {0}")]
    SyntaxError(String),
    #[error("ParserError: {}", .0.description())]
    ParserError(ErrorKind),
    #[error("InvaildNumber: {0}")]
    InvaildNumber(#[from] ParseIntError),
    #[error("Nothing to parse")]
    Empty,
    #[error("Unknown error")]
    Unknown,
}
#[derive(Debug, Error)]
pub struct Error {
    #[source] error: ParseError,
    at: Option<usize>,
}

impl ParseError {
    pub fn syntax_error<S1: fmt::Display, S2: fmt::Display>(content: S1, description: S2) -> Self {
        Self::SyntaxError(format!("{} : {}", content, description))
    }
    pub fn unexpected_token<S: fmt::Display>(msg: S) -> Self {
        Self::syntax_error(msg, "unexpected token")
    }
    pub fn from_error_kind(ek: ErrorKind) -> Self {
        Self::ParserError(ek)
    }
}
impl<I> From<nom::Err<(I, ErrorKind)>> for ParseError {
    fn from(e: nom::Err<(I, ErrorKind)>) -> Self {
        match e {
            nom::Err::Incomplete(_) => unreachable!(),
            nom::Err::Error((_, ek)) | nom::Err::Failure((_, ek)) => Self::from_error_kind(ek),
        }
    }
}
impl Error {
    pub fn new(error: ParseError, at: usize) -> Self {
        Self { error, at: Some(at) }
    }
    pub fn from_parse_error(error: ParseError) -> Self {
        Self { error, at: None }
    }
    pub fn error(&self) -> &ParseError {
        &self.error
    }
    #[allow(dead_code)]
    pub(crate) fn set_at_line(&mut self, line: usize) {
        self.at.replace(line);
    }
}
impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result { 
        match self.at {
            Some(l) => write!(formatter, "{} at line {}", self.error, l),
            None => write!(formatter, "{}", self.error),
        }
    }
}
impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Self::from_parse_error(e)
    }
}