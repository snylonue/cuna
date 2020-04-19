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
        ($(nom::bytes::complete::tag_no_case($heads),)*)
    };
}

pub mod time;

use failure::Error;
use nom::IResult;
use nom::error::ErrorKind;
use nom::bytes::complete::tag_no_case as tag;
use nom::bytes::complete::take_until;
use nom::sequence::delimited;
use nom::branch::alt;
use nom::Err as NomErr;
use std::fs::File;
use std::collections::BTreeMap;
use std::io::Read;
use std::iter::FromIterator;
use std::str::FromStr;

type HanaResult<T> = Result<T, Error>;

#[derive(Debug)]
#[derive(Debug, Default)]
pub struct Header {
    pub title: Option<String>,
    pub performer: Option<String>,
    pub songwriter: Option<String>,
#[derive(Debug, Default)]
pub struct Header {
    pub title: Option<Vec<String>>,
    pub performer: Option<Vec<String>>,
    pub songwriter: Option<Vec<String>>,
    pub catalog: Option<u64>,
    pub cdtextfile: Option<String>,
}
pub struct Comment(pub Vec<String>);

impl FromStr for Header {
    type Err = Error;

    /// Note that the function will ignore repeated commands and only return the last one
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_header(s)
    }
}
impl Comment {
    pub fn new(s: &str) -> Self {
        s.lines()
            .filter_map(|s| parse_line(s, "REM")
                .ok()
                .map(|(c, _)| c.trim())
            )
            .collect()
    }
}
impl<S: Into<String>> FromIterator<S> for Comment {
    fn from_iter<T: IntoIterator<Item=S>>(iter: T) -> Self {
        Self(iter.into_iter().map(Into::into).collect())
    }
}

fn parse_line<'a>(line: &'a str, head: &str) -> IResult<&'a str, &'a str> {
    tag(head)(line)
}
fn parse_quote(content: &str) -> IResult<&str, &str>  {
    delimited(tag(r#"""#), take_until(r#"""#), tag(r#"""#))(content)
fn quote() -> impl Fn(&str) -> IResult<&str, &str, (&str, ErrorKind)> {
    |i: &str| delimited(tag(r#"""#), take_until(r#"""#), tag(r#"""#))(i)
}
fn quotec(content: &str) -> IResult<&str, &str>  {
    quote()(content)
}
fn parse_comments(s: &str) -> (Comment, String) {
    let comments = Comment::new(s);
    let s_without_comments = s.lines()
        .filter(|s| parse_line(s, "REM ").is_err())
        .collect::<Vec<&str>>()
        .join("\n");
    (comments, s_without_comments)
}
/// Split up a cue file into `track_info` and `header`
/// Returns a tuple of (track_info, header) (in IResult) if successful
fn split_cue(s: &str) -> IResult<&str, String> {
    let (mut file, headers) = take_until("FILE")(s)?;
    let mut headers = headers.to_string();
    let mut split_file = file.rsplitn(2, "\n  ");
    let rest_header = split_file.next()
        .map(|s| s.splitn(2, '\n').last())
        .flatten();
    match rest_header {
        Some(rest_header) if rest_header != "" => {
            headers += rest_header;
            file = take_until(rest_header)(file)?.1;
        },
        _ => {},
    }
    Ok((file, headers))
}
fn parse_header(s: &str) -> HanaResult<Header> {
    let mut headers = BTreeMap::new();
    for line in s.lines() {
        match alt::<_, _, (_, ErrorKind), _>(tags!("title", "performer", "songwriter", "catalog", "cdtextfile"))(line) {
            Ok((content, command)) => match quotec(content.trim()) {
                Ok((_, content)) | Err(NomErr::Error((content, _))) => headers.entry(command.trim().to_ascii_lowercase()).or_insert_with(|| Vec::with_capacity(1)).push(content.to_owned()),
                _ => return Err(failure::err_msg("Unexcept error while parsing header")),
            }
            _ => {}
        }
    }
    let [title, performer, songwriter, catalog, cdtextfile] = get!(headers,
        (title, performer, songwriter, catalog, cdtextfile));
    let header = Header {
        title,
        performer,
        songwriter,
        catalog: match catalog {
            Some(s) if s[0].len() == 13 => Some(s[0].parse()?),
            Some(_) => return Err(failure::err_msg("Invaild catalog")),
            None => None,
        },
        cdtextfile: cdtextfile.map(|mut v| v.pop()).flatten(),
    };
    Ok(header)
}
