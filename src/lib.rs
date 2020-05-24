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
        nom::branch::alt::<_, _, (_, nom::error::ErrorKind), _>(($($crate::utils::keyword($heads),)*))
    };
}

pub mod utils;
pub mod time;
pub mod header;
pub mod track;
pub mod comment;

use anyhow::Error;
use std::fs;
use std::io::Read;
use std::str::FromStr;
use crate::track::Track;
use crate::track::TrackInfo;
use crate::header::Header;
use crate::comment::Comment;

#[derive(Debug, Clone, Default)]
pub struct CueSheet {
    pub header: Header,
    pub tracks: Vec<TrackInfo>,
    pub comments: Comment,
}

impl CueSheet {
    pub fn new(header: Header, tracks: Vec<TrackInfo>, comments: Comment) -> Self {
        Self { header, tracks, comments }
    }
    pub fn push_track_info(&mut self, track: TrackInfo) {
        self.tracks.push(track);
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
        .filter(|s| utils::keyword("REM")(s).is_err())
        .collect::<Vec<&str>>()
        .join("\n");
    (comments, s_without_comments)
}