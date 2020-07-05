use thiserror::Error;
use nom::error::ErrorKind;
use std::fmt;
use std::num::ParseIntError;
use std::io;

#[derive(Debug, Error)]
pub enum ParseError {
    /// There is something wrong in the cue sheet
    #[error("SyntaxError: {0}")]
    SyntaxError(String),
    /// Errors from nom
    #[error("ParserError: {}", .0.description())]
    ParserError(ErrorKind),
    /// There is nothing to parse
    #[error("Nothing to parse")]
    Empty,
    /// Fails to read a file
    #[error("IoError: {0}")]
    IoError(#[from] io::Error),
}
#[derive(Debug, Error)]
pub struct Error {
    #[source] error: ParseError,
    at: Option<usize>,
}

impl ParseError {
    pub fn syntax_error<S1: fmt::Display, S2: fmt::Display>(content: S1, description: S2) -> Self {
        Self::err_msg(format!("{} : {}", content, description))
    }
    pub fn unexpected_token<S: fmt::Display>(msg: S) -> Self {
        Self::syntax_error(msg, "unexpected token")
    }
    pub fn err_msg<S: fmt::Display>(msg: S) -> Self {
        Self::SyntaxError(msg.to_string())
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
impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        Self::err_msg(e)
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
    pub fn at(&self) -> &Option<usize> {
        &self.at
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
impl<E: Into<ParseError>> From<E> for Error {
    fn from(e: E) -> Self {
        Self::from_parse_error(e.into())
    }
}