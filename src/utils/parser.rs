use failure::format_err;
use nom::bytes::complete::take_until;
use nom::error::ErrorKind;
use std::vec::IntoIter;
use crate::HanaResult;
use crate::CueSheet;

#[derive(Debug, Clone)]
pub(crate) enum Command<'a> {
    Rem(&'a str),
    Title(&'a str),
    Performer(&'a str),
    Songwriter(&'a str),
    Catalog(&'a str),
    Cdtextfile(&'a str),
    File(&'a str, &'a str),
    Track(&'a str, &'a str),
    Index(&'a str, &'a str),
    Pregap(&'a str),
    Postgap(&'a str),
    Isrc(&'a str),
    Flag(&'a str),
}
#[derive(Debug, Clone)]
pub(crate) struct Line<'a> {
    command: Command<'a>,
    indentations: usize,
    current_line: usize,
}
#[derive(Debug, Clone)]
pub(crate) struct Lines<'a> {
    lines: Vec<Line<'a>>,
}
#[derive(Debug, Clone)]
pub(crate) struct LinesIter<'a> {
    inner: &'a Lines<'a>,
    current_line: usize,
}

impl<'a> Command<'a> {
    pub fn new(s: &'a str) -> HanaResult<Self> {
        let (content, command) = take_until::<_, _, (_, ErrorKind)>(" ")(s)
            .map_err(|_| format_err!("Invaild command {}", s))?;
        let map_err = |_| format_err!("Invaild command {} {}", content, command);
        let (rest, content) = super::quote_opt(content.trim()).map_err(map_err)?;
        match command.to_ascii_lowercase().as_ref() {
            "rem" => Ok(Self::Rem(content)),
            "title" => Ok(Self::Title(content)),
            "performer" => Ok(Self::Performer(content)),
            "songwriter" => Ok(Self::Songwriter(content)),
            "catalog" => Ok(Self::Catalog(content)),
            "cdtextfile" => Ok(Self::Cdtextfile(content)),
            "file" => Ok(Self::File(rest, content)),
            "track" => Ok(Self::Track(rest, content)),
            "index" => Ok(Self::Index(rest, content)),
            "pregap" => Ok(Self::Pregap(content)),
            "postgap" => Ok(Self::Postgap(content)),
            "isrc" => Ok(Self::Isrc(content)),
            "flag" => Ok(Self::Flag(content)),
            _ => Err(format_err!("UnKnown command `{}`", command)),
        }
    }
}
impl<'a> Line<'a> {
    pub fn new(s: &'a str, current_line: usize) -> HanaResult<Self> {
        let indentations = super::indentation_count(s);
        let command = Command::new(&s.trim()).map_err(|e| format_err!("{} at line {}", e, current_line + 1))?;
        Ok( Self { command, indentations, current_line })
    }
}
impl<'a> Lines<'a> {
    pub fn new(s: &'a str) -> HanaResult<Self> {
        let lines = s.lines()
            .enumerate()
            .map(|(line, s)| Line::new(s, line))
            .collect::<Result<_, _>>()?;
        Ok(Self { lines })
    }
    pub fn len(&self) -> usize {
        self.lines.len()
    }
    pub fn line(&self, l: usize) -> Option<&Line<'a>> {
        self.lines.get(l)
    }
    #[allow(dead_code)]
    pub fn iter(&self) -> LinesIter {
        LinesIter { inner: &self, current_line: 0 }
    }
}
impl<'a> IntoIterator for Lines<'a> {
    type Item = Line<'a>;
    type IntoIter = IntoIter<Line<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.lines.into_iter()
    }
}
impl<'a> Iterator for LinesIter<'a> {
    type Item = &'a Line<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_line < self.inner.len() {
            self.current_line += 1;
            self.inner.line(self.current_line - 1)
        } else {
            None
        }
    }
}

pub fn parse(s: &str) -> HanaResult<CueSheet> {
    let lines = Lines::new(s)?;
    let mut cue = CueSheet::default();
    dbg!(lines);
    unimplemented!()
}