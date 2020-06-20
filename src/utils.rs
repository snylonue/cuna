use nom::IResult;
use nom::error::ErrorKind;
use nom::bytes::complete::tag_no_case as tag;
use nom::bytes::complete::take_until;
use nom::bytes::complete::take_while_m_n;
use nom::sequence::delimited;
use nom::sequence::terminated;
use nom::character::complete::space0;
use nom::branch::alt;
use nom::combinator::rest;

pub fn keyword<'a, 'b: 'a>(kd: &'b str) -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
    move |i: &str| terminated(tag(kd), tag(" "))(i)
}
pub fn keywordc<'a, 'b: 'a>(kd: &'b str, content: &'a str) -> IResult<&'a str, &'a str> {
    keyword(kd)(content)
}
pub fn quote(content: &str) -> IResult<&str, &str>  {
    delimited(tag(r#"""#), take_until(r#"""#), tag(r#"""#))(content)
}
pub fn quote_opt(content: &str) -> IResult<&str, &str> {
    alt((quote, rest))(content)
}
pub fn split_space(content: &str) -> IResult<&str, &str> {
    terminated(take_until(" "), tag(" "))(content)
}
pub(crate) fn take_digit2(s: &str) -> IResult<&str, &str, (&str, ErrorKind)> {
    take_while_m_n(2, 2, |c: char| c.is_digit(10))(s)
}
pub(crate) fn indentation_count(s: &str) -> usize {
    space0::<_, (_, ErrorKind)>(s).map(|(_, o)| o.len()).unwrap()
}