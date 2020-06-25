use std::collections::VecDeque;
use std::fmt;
use crate::error::ParseError;
use crate::error::Error;
use crate::CueSheet;
use crate::track::TrackInfo;
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
    pub fn new(s: &'a str, line: usize) -> Result<Self, Error> {
        let indentations = utils::indentation_count(s);
        let command = Command::new(&s.trim()).map_err(
            |e| Error::new(e, line)
        )?;
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
    pub fn new(s: &'a str) -> Result<Self, Error> {
        let lines = s.lines()
            .enumerate()
            .map(|(line, content)| Line::new(content, line + 1))
            .collect::<Result<VecDeque<_>, Error>>()?;
        Ok(Self { lines, sheet: CueSheet::default() })
    }
    pub fn current_line(&self) -> Option<&Line> {
        self.lines.front()
    }
    pub fn parse_next_line(&mut self) -> Result<Line<'_>, ParseError> {
        let current_line = match self.lines.pop_front() {
            Some(cl) => cl,
            None => return Err(ParseError::Empty),
        };
        let command = current_line.command();
        match *command {
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
                return Err(ParseError::syntax_error(command, "multiple `CATALOG` commands is not allowed"));
            }
            Command::Cdtextfile(s) => {
                self.sheet.header.set_cdtextfile(s.to_owned());
            },
            Command::File(name, format) => {
                self.sheet.push_track_info(TrackInfo::new(name.to_owned(), format.to_owned()));
            },
            Command::Track(..) => {
                match self.sheet.last_track_info_mut() {
                    Some(tk) => tk.push_track(command.to_string().parse()?),
                    None => return Err(ParseError::syntax_error(command, "Multiple `CATALOG` commands is not allowed")),
                }
            },
            Command::Index(..) => match self.sheet.last_track_mut() {
                Some(tk) if tk.postgap.is_none() => {
                    tk.push_index(command.to_string().parse()?)
                },
                Some(_) => return Err(ParseError::syntax_error(command, "Command `INDEX` should be before `POSTGAP`")),
                None => return Err(ParseError::unexpected_token("INDEX")),
            }
            Command::Pregap(duration) => match self.sheet.last_track_mut() {
                Some(tk) if tk.index.is_empty() && tk.pregap.is_none() => {
                    tk.set_pregep(duration.parse()?);
                },
                Some(tk) if !tk.index.is_empty() => return Err(ParseError::syntax_error(command, "Command `PREGAP` should be before `INDEX`")),
                Some(tk) if tk.pregap.is_some() => return Err(ParseError::syntax_error(command, "Multiple `PREGAP` commands are not allowed in one `TRACK` scope")),
                _ => return Err(ParseError::unexpected_token("PREGAP")),
            },
            Command::Postgap(duration) => match self.sheet.last_track_mut() {
                Some(tk) if tk.postgap.is_none() => {
                    tk.set_postgep(duration.parse()?);
                },
                Some(_) => return Err(ParseError::syntax_error(command, "Multiple `POSTGAP` commands are not allowed in one `TRACK` scope")),
                None => return Err(ParseError::unexpected_token("POSTGAP")),
            },
            Command::Isrc(s) => match self.sheet.last_track_mut() {
                Some(tk) if tk.isrc.is_none() => {
                    tk.set_isrc(s.to_owned());
                },
                Some(_) => return Err(ParseError::syntax_error(command, "Multiple `ISRC` commands are not allowed in one `TRACK` scope")),
                None => return Err(ParseError::unexpected_token("ISRC")),
            },
            Command::Flags(s) => match self.sheet.last_track_mut() {
                Some(tk) if tk.flags.is_none() => tk.push_flags(s.split(' ')),
                Some(_) => return Err(ParseError::syntax_error(command, "Multiple `FLAGS` commands are not allowed in one `TRACK` scope")),
                None => return Err(ParseError::unexpected_token("FLAGS")),
            }
        }
        Ok(current_line)
    }
    pub fn parse(mut self) -> Result<CueSheet, Error> {
        let mut current_line = 0;
        loop {
            match self.parse_next_line() {
                Ok(cl) => current_line = cl.line,
                Err(e) => match e {
                    ParseError::Empty => break,
                    _ => return Err(Error::new(e, current_line)),
                }
            }
        }
        Ok(self.sheet)
    }
}