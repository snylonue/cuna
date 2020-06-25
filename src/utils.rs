use nom::IResult;
use nom::error::ErrorKind;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::sequence::delimited;
use nom::sequence::terminated;
use nom::character::complete::space0;
use nom::character::complete::digit0;
use nom::branch::alt;
use nom::combinator::rest;
use nom::combinator::verify;
use nom::combinator::map;

pub fn keyword<'a, 'b: 'a>(kd: &'b str) -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
    move |i: &str| terminated(tag_no_case(kd), tag(" "))(i)
}
pub fn keywordc<'a, 'b: 'a>(kd: &'b str, content: &'a str) -> IResult<&'a str, &'a str> {
    keyword(kd)(content)
}
pub fn quote(content: &str) -> IResult<&str, &str>  {
    delimited(
        tag(r#"""#),
        take_until(r#"""#),
        tag(r#"""#)
    )(content)
}
pub fn quote_opt(content: &str) -> IResult<&str, &str> {
    verify(
        alt((
            quote,
            rest
        )),
        |s: &str| !s.contains('"')
    )(content)
}
pub fn token(content: &str) -> IResult<&str, &str> {
    terminated(take_until(" "), tag(" "))(content)
}
pub fn number(n: usize) -> impl Fn(&str) -> IResult<&str, u8> {
    move |i: &str| map(
        verify(
            digit0, 
            |d: &str| d.len() == n
        ),
        |d: &str| d.parse().unwrap()
    )(i)
}
pub(crate) fn indentation_count(s: &str) -> usize {
    space0::<_, (_, ErrorKind)>(s).map(|(_, o)| o.len()).unwrap()
}