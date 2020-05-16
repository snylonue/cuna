use anyhow::Result;
use nom::bytes::complete::take_until;
use nom::Err as NomErr;
use nom::error::ErrorKind;
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) enum State {
    Outer,
    File,
    Track,
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
pub(crate) struct Parser<'a> {
    state: State,
    lines: Lines<'a>,
    current_line: usize,
    sheet: CueSheet,
}

impl<'a> Command<'a> {
    pub fn new(s: &'a str) -> Result<Self> {
        let (content, command) = take_until(" ")(s)
            .map_err(|_: NomErr<(_, ErrorKind)>| anyhow::anyhow!("Invaild command {}", s))?;
        let (rest, content) = super::quote_opt(content.trim())
            .map_err(|_| anyhow::anyhow!("Invaild command {} {}", content, command))?;
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
            _ => Err(anyhow::anyhow!("UnKnown command `{}`", command)),
        }
    }
}
impl<'a> Line<'a> {
    pub fn new(s: &'a str, current_line: usize) -> Result<Self> {
        let indentations = super::indentation_count(s);
        let command = Command::new(&s.trim()).map_err(|e| anyhow::anyhow!("{} at line {}", e, current_line + 1))?;
        Ok( Self { command, indentations, current_line })
    }
}
#[allow(dead_code)]
impl<'a> Lines<'a> {
    pub fn new(s: &'a str) -> Result<Self> {
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
}

#[allow(unused_mut)]
#[allow(unused_variables)]
pub fn parse(s: &str) -> Result<CueSheet> {
    let lines = Lines::new(s)?;
    let mut cue = CueSheet::default();
    dbg!(lines);
    unimplemented!()
}