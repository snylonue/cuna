use nom::IResult;
use nom::error::ErrorKind;
use nom::bytes::complete::tag_no_case as tag;
use nom::bytes::complete::take_until;
use nom::sequence::delimited;
use nom::character::complete::space0;

pub(crate) fn parse_line<'a>(line: &'a str, head: &str) -> IResult<&'a str, &'a str> {
    tag(head)(line)
}
pub(crate) fn quote() -> impl Fn(&str) -> IResult<&str, &str, (&str, ErrorKind)> {
    |i: &str| delimited(tag(r#"""#), take_until(r#"""#), tag(r#"""#))(i)
}
pub fn quotec(content: &str) -> IResult<&str, &str>  {
    quote()(content)
}
pub(crate) fn indentation_num(s: &str) -> usize {
    space0::<_, (_, ErrorKind)>(s).map(|(_, o)| o.len()).unwrap()
}
fn find_indentation<'a, I>(line: I, init_indents: usize) -> Vec<usize>
    where I: IntoIterator<Item = &'a str> + Clone
{
    let mut lines = line.clone().into_iter().enumerate();
    let mut indexs = Vec::new();
    while let Some((line_st, _)) = lines.find(|(_, s)| indentation_num(s) > init_indents) {
        // line_st needs checking if it is at the first line
        indexs.push(line_st);
        indexs.extend(lines.find(|(_, s)| indentation_num(s) == init_indents).map(|line_ed| line_ed.0));
    }
    if indexs.len() % 2 == 1 {
        indexs.push(line.clone().into_iter().count());
    }
    indexs
}
/// Splits a str into two part according to if there's an indentation at a line
/// Returns a tuple of them in Vecs like `(no_indentation, with_indentation)`
pub(crate) fn scope(s: &str) -> (Vec<&str>, Vec<Vec<&str>>) {
    if s == "" {
        return (Vec::new(), Vec::new());
    }
    let init_indents = indentation_num(s.lines().next().unwrap());
    let indexs = find_indentation(s.lines(), init_indents);
    let mut lines = s.lines().collect::<Vec<_>>();
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