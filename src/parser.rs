use anyhow::Result;
use std::collections::VecDeque;
use std::fmt;
use crate::error::ParseError;
use crate::CueSheet;
use crate::track::TrackInfo;
use crate::track::Track;
use crate::track::Index;
use crate::utils;

#[derive(Debug, Clone)]
pub enum Command<'a> {
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
#[derive(Debug, Clone)]
pub struct Line<'a> {
    command: Command<'a>,
    indentations: usize,
    line: usize,
}
#[derive(Debug, Clone)]
pub struct Parser<'a> {
    lines: VecDeque<Line<'a>>,
    sheet: CueSheet,
}

impl<'a> Command<'a> {
    pub fn new(s: &'a str) -> Result<Self, ParseError> {
        let s = s.trim();
        let (content, command) = utils::token(s).map_err(
            |_| ParseError::syntax_error(s, "missing arguments")
        )?;
        let (rest, content) = utils::quote_opt(content.trim()).map_err(
            |e| match e {
                nom::Err::Error((es, _)) => ParseError::syntax_error(es, "invaild string"),
                nom::Err::Failure((_, ek)) => ParseError::ParserError(ek),
                nom::Err::Incomplete(_) => unreachable!(),
        })?;
        match command.to_ascii_lowercase().as_ref() {
            "rem" => Ok(Self::Rem(content)),
            "title" => Ok(Self::Title(content)),
            "performer" => Ok(Self::Performer(content)),
            "songwriter" => Ok(Self::Songwriter(content)),
            "catalog" => Ok(Self::Catalog(content)),
            "cdtextfile" => Ok(Self::Cdtextfile(content)),
            "file" => Ok(Self::File(content, rest.trim())),
            "track" => {
                let (format, id) = utils::token(content)
                    .map_err(|_| ParseError::syntax_error(command, "missing arguments"))?;
                Ok(Self::Track(id, format))
            },
            "index" => {
                let (duration, id) = utils::token(content)
                    .map_err(|_| ParseError::syntax_error(command, "missing arguments"))?;
                Ok(Self::Index(id, duration))
            },
            "pregap" => Ok(Self::Pregap(content)),
            "postgap" => Ok(Self::Postgap(content)),
            "isrc" => Ok(Self::Isrc(content)),
            "flag" => Ok(Self::Flags(content)),
            _ => Err(ParseError::unexpected_token(command)),
        }
    }
}
impl<'a> fmt::Display for Command<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result { 
        let command = match *self {
            Command::Rem(c) => format!("REM {}", c),
            Command::Title(c) => format!(r#"TITLE "{}""#, c),
            Command::Performer(c) => format!(r#"PERFORMER "{}""#, c),
            Command::Songwriter(c) => format!(r#"SONGWRITER "{}""#, c),
            Command::Catalog(c) => format!("CATALOG {}", c),
            Command::Cdtextfile(c) => format!(r#"CDTEXTFILE "{}""#, c),
            Command::File(name, tp) => format!(r#"FILE "{}" {}"#, name, tp),
            Command::Track(id, format) => format!("TRACK {} {}", id, format),
            Command::Index(id, duration) => format!("INDEX {} {}", id, duration),
            Command::Pregap(c) => format!("PREGAP {}", c),
            Command::Postgap(c) => format!("POSTGAP {}", c),
            Command::Isrc(c) => format!("ISRC {}", c),
            Command::Flags(c) => format!("FLAGS {}", c),
        };
        write!(formatter, "{}", command)
     }
}
impl<'a> Line<'a> {
    pub fn new(s: &'a str, line: usize) -> Result<Self> {
        let indentations = utils::indentation_count(s);
        let command = Command::new(&s.trim()).map_err(|e| anyhow::anyhow!("{} at line {}", e, line))?;
        Ok( Self { command, indentations, line })
    }
    pub fn command(&self) -> &Command {
        &self.command
    }
    pub fn line(&self) -> usize {
        self.line
    }
    pub fn indentations(&self) -> usize {
        self.indentations
    }
}
impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Result<Self> {
        let lines = s.lines()
            .enumerate()
            .map(|(line, content)| Line::new(content, line + 1))
            .collect::<Result<VecDeque<_>>>()?;
        Ok(Self { lines, sheet: CueSheet::default() })
    }
    pub fn current_line(&self) -> Option<&Line> {
        self.lines.front()
    }
    pub fn parse_next_line(&mut self) -> Result<Line<'_>> {
        let current_line = match self.lines.pop_front() {
            Some(cl) => cl,
            None => anyhow::bail!("Nothing to parse"),
        };
        match *current_line.command() {
            Command::Rem(s) => self.sheet.comments.push(s.to_owned()),
            Command::Title(s) => match self.sheet.last_track_mut() {
                Some(tk) => tk.push_title(s.to_owned()),
                None => self.sheet.header.push_title(s.to_owned()),
            },
            Command::Performer(s) => match self.sheet.last_track_mut() {
                Some(tk) => tk.push_performer(s.to_owned()),
                _ => self.sheet.header.push_performer(s.to_owned()),
            },
            Command::Songwriter(s) => match self.sheet.last_track_mut() {
                Some(tk) => tk.push_songwriter(s.to_owned()),
                _ => self.sheet.header.push_songwriter(s.to_owned()),
            },
            Command::Catalog(s) => if self.sheet.header.catalog.is_none() {
                self.sheet.header.set_catalog(s.parse()?)?;
            } else {
                return Err(anyhow::format_err!("Line {}: `CATALOG {}`: multiple `CATALOG` commands is not allowed", current_line.line(), s));
            }
            Command::Cdtextfile(s) => {
                self.sheet.header.set_cdtextfile(s.to_owned());
            },
            Command::File(name, format) => {
                self.sheet.push_track_info(TrackInfo::new(name.to_owned(), format.to_owned()));
            },
            Command::Track(id, format) => {
                match self.sheet.last_track_info_mut() {
                    Some(tk) => tk.push_track(Track::new(id.parse()?, format.to_owned())?),
                    None => anyhow::bail!("Multiple `CATALOG` commands is not allowed"),
                }
            },
            Command::Index(id, duration) => match self.sheet.last_track_mut() {
                Some(tk) if tk.postgap.is_none() => {
                    tk.push_index(Index::new(id.parse()?, duration.parse()?)?)
                },
                Some(_) => anyhow::bail!("Command `INDEX` should be before `POSTGAP`"),
                None => anyhow::bail!("`Index {} {}`: Unexpected command", id, duration),
            }
            Command::Pregap(duration) => match self.sheet.last_track_mut() {
                Some(tk) if tk.index.is_empty() && tk.pregap.is_none() => {
                    tk.set_pregep(duration.parse()?);
                },
                Some(tk) if !tk.index.is_empty() => anyhow::bail!("Command `PREGAP` should be before `INDEX`"),
                Some(tk) if tk.pregap.is_some() => anyhow::bail!("Multiple `PREGAP` commands are not allowed in one `TRACK` scope"),
                _ => anyhow::bail!("`PREGAP {}`: Unexpected command", duration),
            },
            Command::Postgap(duration) => match self.sheet.last_track_mut() {
                Some(tk) if tk.postgap.is_none() => {
                    tk.set_postgep(duration.parse()?);
                },
                Some(_) => anyhow::bail!("Multiple `POSTGAP` commands are not allowed in one `TRACK` scope"),
                None => anyhow::bail!("`POSTGAP {}`: Unexpected command", duration),
            },
            Command::Isrc(s) => match self.sheet.last_track_mut() {
                Some(tk) if tk.isrc.is_none() => {
                    tk.set_isrc(s.to_owned());
                },
                Some(_) => anyhow::bail!("Multiple `ISRC` commands are not allowed in one `TRACK` scope"),
                None => anyhow::bail!("`ISRC {}`: Unexpected command", s),
            },
            Command::Flags(s) => match self.sheet.last_track_mut() {
                Some(tk) if tk.flags.is_none() => tk.push_flags(s.split(' ')),
                Some(_) => anyhow::bail!("Multiple `FLAGS` commands are not allowed in one `TRACK` scope"),
                None => anyhow::bail!("`FLAGS {}`: Unexpected command", s),
            }
        }
        Ok(current_line)
    }
    pub fn parse(mut self) -> Result<CueSheet> {
        let mut current_line = 0;
        while !self.lines.is_empty() {
            match self.parse_next_line() {
                Ok(cl) => current_line = cl.line,
                Err(e) => anyhow::bail!("Error at line {}: {}", current_line + 1, e),
            }
        }
        Ok(self.sheet)
    }
}