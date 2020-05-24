use anyhow::Result;
use std::collections::VecDeque;
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
    Flags(&'a str),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) enum State {
    Global,
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
pub(crate) struct Parser<'a> {
    state: State,
    lines: VecDeque<Line<'a>>,
    current_line: usize,
    sheet: CueSheet,
}

#[allow(dead_code)]
impl<'a> Command<'a> {
    pub fn new(s: &'a str) -> Result<Self> {
        let (content, command) = super::split_space(s)
            .map_err(|_| anyhow::anyhow!("Invaild command {}", s))?;
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
            "track" => {
                let (format, id) = super::split_space(content)
                    .map_err(|_| anyhow::anyhow!("Invaild command {}", content))?;
                Ok(Self::Track(id, format))
            },
            "index" => {
                let (duration, id) = super::split_space(content)
                    .map_err(|_| anyhow::anyhow!("Invaild command {}", content))?;
                Ok(Self::Index(id, duration))
            },
            "pregap" => Ok(Self::Pregap(content)),
            "postgap" => Ok(Self::Postgap(content)),
            "isrc" => Ok(Self::Isrc(content)),
            "flag" => Ok(Self::Flags(content)),
            _ => Err(anyhow::anyhow!("UnKnown command `{}`", command)),
        }
    }
    pub fn may_in_global_scope(&self) -> bool {
        match *self {
            Self::Rem(_) => true,
            Self::Title(_) => true,
            Self::Performer(_) => true,
            Self::Songwriter(_) => true,
            Self::Catalog(_) => true,
            Self::Cdtextfile(_) => true,
            Self::File(..) => true,
            _ => false,
        }
    }
    pub fn may_in_file_scope(&self) -> bool {
        match *self {
            Self::Rem(_) => true,
            Self::Track(..) => true,
            _ => false,
        }
    }
    pub fn may_in_track_scope(&self) -> bool {
        match *self {
            Self::Rem(_) => true,
            Self::Title(_) => true,
            Self::Performer(_) => true,
            Self::Songwriter(_) => true,
            Self::Catalog(_) => true,
            Self::Cdtextfile(_) => true,
            Self::Index(..) => true,
            Self::Pregap(_) => true,
            Self::Postgap(_) => true,
            Self::Isrc(_) => true,
            Self::Flags(_) => true,
            _ => false,
        }
    }
}
#[allow(dead_code)]
impl<'a> Line<'a> {
    pub fn new(s: &'a str, current_line: usize) -> Result<Self> {
        let indentations = super::indentation_count(s);
        let command = Command::new(&s.trim()).map_err(|e| anyhow::anyhow!("{} at line {}", e, current_line + 1))?;
        Ok( Self { command, indentations, current_line })
    }
}

#[allow(unused_mut)]
#[allow(unused_variables)]
pub fn parse(s: &str) -> Result<CueSheet> {
    unimplemented!()
}