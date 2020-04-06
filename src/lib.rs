use nom::IResult;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use std::fs::File;
use std::io::Read;
use std::iter::FromIterator;

#[derive(Debug)]
pub struct Comment(pub Vec<String>);

impl<S: Into<String>> FromIterator<S> for Comment {
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        Self(iter.into_iter().map(Into::into).collect())
    }
}

fn parse_line<'a>(line: &'a str, head: &str) -> IResult<&'a str, &'a str> {
    tag(head)(line)
}
fn parse_comments(s: &str) -> (Comment, String) {
    let parse_comment = |s| parse_line(s, "REM ");
    let comments = s.lines()
        .filter_map(|s| parse_comment(s)
            .ok()
            .map(|(c, _)| c)
        )
        .collect::<Comment>();
    let s_without_comments = s.lines()
        .filter(|s| parse_comment(s).is_err())
        .collect::<Vec<&str>>()
        .join("\n");
    (comments, s_without_comments)
}
