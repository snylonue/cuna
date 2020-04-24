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
use nom::character::complete::space0;
use nom::Err as NomErr;
use std::fs::File;
use std::collections::BTreeMap;
use std::io::Read;
use std::iter::FromIterator;
use std::str::FromStr;
use crate::time::Duration;

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
fn indentation_num(s: &str) -> usize {
    space0::<_, (_, ErrorKind)>(s).map(|(_, o)| o.len()).unwrap()
}
/// Splits a str into two part according to if there's an indentation at a line
/// Returns a tuple of them in Vecs like (no_indentation, with_indentation)
fn scope(s: &str) -> (Vec<&str>, Vec<Vec<&str>>) {
    if s == "" {
        return (Vec::new(), Vec::new());
    }
    let init_indents = indentation_num(s.lines().next().unwrap());
    let mut lines = s.lines().enumerate();
    let mut indexs = Vec::new();
    while let Some((line_st, _)) = lines.find(|(_, s)| indentation_num(s) > init_indents) {
        // line_st needs checking if it is at the first line
        indexs.push(line_st);
        if let Some((line_ed, _)) = lines.find(|(_, s)| indentation_num(s) == init_indents) {
            indexs.push(line_ed);
        }
    }
    let mut lines = s.lines().collect::<Vec<_>>();
    if indexs.len() % 2 == 1 {
        indexs.push(lines.len());
    }
    let mut in_scope = Vec::new();
    let mut out_scope = Vec::new();
    let mut indexs_iter = indexs.iter();
    let mut current_index = 0;
    while let Some(i) = indexs_iter.next() {
        let rest = lines.split_off(i - current_index - 1);
        current_index = i.clone();
        out_scope.append(&mut lines);
        lines = rest;
        if let Some(j) = indexs_iter.next() {
            let rest = lines.split_off(j - current_index + 1);
            let mut scope = Vec::new();
            scope.append(&mut lines);
            in_scope.push(scope);
            lines = rest;
            current_index = j.clone();
        }
    }
    (out_scope, in_scope)
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
