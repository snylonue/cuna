macro_rules! get {
    ($map: expr, ($($key: ident),*)) => {
        [$($map.remove(stringify!($key)),)*]
    };
    ($map: expr, ($($key: ident),*) >> $map_res: expr) => {
        [$($map.remove(stringify!($key)).map($map_res),)*]
    };
}
macro_rules! tags {
    ($($heads: expr),*) => {
        nom::branch::alt::<_, _, (_, nom::error::ErrorKind), _>(($(nom::bytes::complete::tag_no_case($heads),)*))
    };
}

pub mod utils;
pub mod time;
pub mod header;
pub mod filedata;
pub mod comment;

use failure::Error;
use std::fs;
use std::io::Read;
use std::str::FromStr;
use crate::track::FileTrucks;
use crate::header::Header;
use crate::comment::Comment;

type HanaResult<T> = Result<T, Error>;

#[derive(Debug)]
pub struct CueSheet {
    pub header: Header,
    pub tracks: Vec<FileTrucks>,
    pub comments: Comment,
}

fn parse_comments(s: &str) -> (Comment, String) {
    let comments = Comment::new(s);
    let s_without_comments = s.lines()
        .filter(|s| utils::parse_line(s, "REM ").is_err())
        .collect::<Vec<&str>>()
        .join("\n");
    (comments, s_without_comments)
}