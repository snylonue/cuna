use nom::IResult;
use nom::error::ErrorKind;
use nom::bytes::complete::tag_no_case as tag;
use nom::bytes::complete::take_until;
use nom::bytes::complete::take_while_m_n;
use nom::sequence::delimited;
use nom::character::complete::space0;
use nom::branch::alt;
use nom::combinator::rest;
use std::mem;

pub(crate) fn parse_line<'a>(line: &'a str, head: &str) -> IResult<&'a str, &'a str> {
    tag(head)(line)
}
pub fn quote(content: &str) -> IResult<&str, &str>  {
    delimited(tag(r#"""#), take_until(r#"""#), tag(r#"""#))(content)
}
pub fn quote_opt(content: &str) -> IResult<&str, &str> {
    alt((quote, rest))(content)
}
pub(crate) fn take_digit2(s: &str) -> IResult<&str, &str, (&str, ErrorKind)> {
    take_while_m_n(2, 2, |c: char| c.is_digit(10))(s)
}
pub(crate) fn indentation_count(s: &str) -> usize {
    space0::<_, (_, ErrorKind)>(s).map(|(_, o)| o.len()).unwrap()
}
fn find_scopes<'a, I>(line: I, init_indents: usize) -> Vec<usize>
    where I: Iterator<Item = &'a str> + Clone
{
    let mut lines = line.clone().enumerate();
    let mut indexs = Vec::new();
    while let Some((line_st, _)) = lines.find(|(_, s)| indentation_count(s) > init_indents) {
        // line_st needs checking if it is at the first line
        indexs.push(line_st);
        indexs.extend(lines.find(|(_, s)| indentation_count(s) == init_indents).map(|ed| ed.0));
    }
    // there is nothing after the last scope
    if indexs.len() % 2 == 1 {
        indexs.push(line.count());
    }
    indexs
}
/// Splits a str into two part according to if there's an indentation at a line
/// Returns a tuple of them in Vecs like `(no_indentation, with_indentation)`
pub(crate) fn scope<'a, I>(lines: I) -> Option<(Vec<&'a str>, Vec<Vec<&'a str>>)>
    where I: Iterator<Item = &'a str> + Clone
{
    let mut lines_peek = lines.clone().peekable();
    let first_line = if let Some(line) = lines_peek.peek() {
        line
    } else {
        return None
    };
    let init_indents = indentation_count(first_line);
    let indexs = find_scopes(lines, init_indents);
    let mut lines = lines_peek.collect::<Vec<_>>();
    let mut in_scope = Vec::new();
    let mut out_scope = Vec::new();
    let mut indexs_iter = indexs.iter();
    let mut current_index = 0;
    while let Some(i) = indexs_iter.next() {
        let rest = lines.split_off(i - current_index - 1);
        out_scope.append(&mut mem::replace(&mut lines, rest));
        current_index = *i;
        if let Some(j) = indexs_iter.next() {
            let rest = lines.split_off(j - current_index + 1);
            in_scope.push(mem::replace(&mut lines, rest));
            current_index = *j;
        }
    }
    Some((out_scope, in_scope))
}