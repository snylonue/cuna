use nom::IResult;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use std::fs::File;
use std::io::Read;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct Comment(pub Vec<String>);

impl FromIterator<String> for Comment {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

fn parse_line<'a>(line: &'a [u8], head: &str) -> IResult<&'a [u8], &'a [u8]> {
    tag(head)(line)
}
fn parse_comments(s: &str) -> (Comment, String) {
    let parse_comment = |s| parse_line(s, "REM ");
    let comments = s.lines()
        .filter_map(|s| parse_comment(s.as_bytes())
            .ok()
            .map(|(c, _)| String::from_utf8_lossy(c).to_string()))
        .collect::<Comment>();
    let s_without_comments = s.clone()
        .lines()
        .filter(|s| parse_comment(s.trim().as_bytes()).is_err())
        .collect::<Vec<&str>>()
        .join("\n");
    (comments, s_without_comments)
}
