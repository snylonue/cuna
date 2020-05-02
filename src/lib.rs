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
pub mod track;
pub mod comment;

use failure::Error;
use std::fs;
use std::io::Read;
use std::str::FromStr;
use crate::track::FileTracks;
use crate::header::Header;
use crate::comment::Comment;

type HanaResult<T> = Result<T, Error>;

#[derive(Debug, Clone)]
pub struct CueSheet {
    pub header: Header,
    pub tracks: Vec<FileTracks>,
    pub comments: Comment,
}

impl CueSheet {
    pub fn new(header: Header, tracks: Vec<FileTracks>, comments: Comment) -> Self {
        Self { header, tracks, comments }
    }
}
impl FromStr for CueSheet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (comments, rest) = parse_comments(s);
        let (headers, files) = utils::scope(rest.lines()).unwrap();
        let header = header::parse_header_lines(headers)?;
        let tracks = files.into_iter()
            .map(|v| v.into_iter())
            .map(track::parse_filetracks_lines)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { header, tracks, comments })
    }
}
fn parse_comments(s: &str) -> (Comment, String) {
    let comments = Comment::new(s);
    let s_without_comments = s.lines()
        .filter(|s| utils::keywordc(s, "REM ").is_err())
        .collect::<Vec<&str>>()
        .join("\n");
    (comments, s_without_comments)
}
pub fn parse_cue(mut f: fs::File) -> CueSheet {
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    buf = buf.trim_start_matches('﻿').replace("\r\n", "\n"); // remove BOM header
    dbg!(buf.parse::<CueSheet>().unwrap());
    // extract_file(&buf).unwrap();
    // let (_, h) = split_cue(&buf).unwrap();
    // println!("{:#?}", h.parse::<Header>());
    //let (comment, buf) = parse_comments(&buf.trim_start_matches('﻿')); // remove BOM
    //println!("{:#?}", comment);
    //println!("{}", buf);
    unimplemented!()
}