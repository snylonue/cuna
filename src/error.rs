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
    pub fn syntax_error(content: impl fmt::Display, description: impl fmt::Display) -> Self {
        Self::err_msg(format!("{} : {}", content, description))
    }
    pub fn unexpected_token(msg: impl fmt::Display) -> Self {
        Self::syntax_error(msg, "unexpected token")
    }
    pub fn err_msg(msg: impl fmt::Display) -> Self {
        Self::SyntaxError(msg.to_string())
    }
    pub const fn from_error_kind(ek: ErrorKind) -> Self {
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
impl PartialEq for ParseError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::SyntaxError(msg), Self::SyntaxError(msg2)) => msg == msg2,
            (Self::ParserError(e), Self::ParserError(e2)) => e == e2,
            (Self::Empty, Self::Empty) => true,
            (Self::IoError(e), Self::IoError(e2)) => e.kind() == e2.kind(),
            _ => false,
        }
    }   
}
impl Error {
    pub const EMPTY: Self = Self::from_parse_error(ParseError::Empty);

    pub const fn new(error: ParseError, at: usize) -> Self {
        Self { error, at: Some(at) }
    }
    pub const fn from_parse_error(error: ParseError) -> Self {
        Self { error, at: None }
    }
    pub const fn kind(&self) -> &ParseError {
        &self.error
    }
    #[deprecated(note = "Please use Error::pos() instead")]
    pub const fn at(&self) -> &Option<usize> {
        &self.at
    }
    pub const fn pos(&self) -> Option<usize> {
        self.at
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
impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.kind() == other.kind()
    }
}