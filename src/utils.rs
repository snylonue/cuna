use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::digit0;
use nom::combinator::map_res;
use nom::combinator::rest;
use nom::combinator::verify;
use nom::sequence::delimited;
use nom::sequence::terminated;
use nom::IResult;
use std::str::FromStr;

pub fn keyword<'a, 'b: 'a>(kd: &'b str) -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
    move |i: &str| terminated(tag_no_case(kd), tag(" "))(i)
}
pub fn quote(content: &str) -> IResult<&str, &str> {
    delimited(tag(r#"""#), take_until(r#"""#), tag(r#"""#))(content)
}
pub fn quote_opt(content: &str) -> IResult<&str, &str> {
    alt((quote, rest))(content)
}
pub fn token(content: &str) -> IResult<&str, &str> {
    terminated(take_until(" "), tag(" "))(content)
}
/// Takes digits and recognizes them as an n digit
pub fn number<N: FromStr>(n: usize) -> impl Fn(&str) -> IResult<&str, N> {
    move |i: &str| map_res(verify(digit0, |d: &str| d.len() == n), |d: &str| d.parse())(i)
}
