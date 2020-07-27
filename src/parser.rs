macro_rules! fail {
    (token $self: ident, $token: expr) => {
        {
            let err = $crate::error::ParseError::unexpected_token($token);
            return Err($crate::error::Error::new(err, $self.line));
        }
    };
    (syntax $self: ident, $cmd: expr, $msg: expr ) => {
        {
            let err = $crate::error::ParseError::syntax_error($cmd, $msg);
            return Err($crate::error::Error::new(err, $self.line));
        }
    }
}

use std::collections::VecDeque;
use std::fmt;
use crate::error::ParseError;
use crate::error::Error;
use crate::CueSheet;
use crate::track::Track;
use crate::track::Index;
use crate::track::TrackInfo;
use crate::time::TimeStamp;
use crate::utils;

macro_rules! trim {
    ($s: expr) => {
        $s.trim_matches('"')
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Command<'a> {
    Rem(&'a str),
    Title(&'a str),
    Performer(&'a str),
    Songwriter(&'a str),
    Catalog(u64),
    Cdtextfile(&'a str),
    File(&'a str, &'a str),
    Track(u8, &'a str),
    Index(u8, TimeStamp),
    Pregap(&'a str),
    Postgap(&'a str),
    Isrc(&'a str),
    Flags(&'a str),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Line<'a> {
    command: Command<'a>,
    line: usize,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Parser<'a> {
    lines: VecDeque<Line<'a>>,
    state: CueSheet,
}

impl<'a> Command<'a> {
    pub fn new(s: &'a str) -> Result<Self, ParseError> {
        let s = match s.trim() {
            "" => return Err(ParseError::Empty),
            ts => ts,
        };
        let (content, command) = match utils::token(s).map(|(cont, cmd)| (cont, cmd.to_ascii_lowercase())) {
            Ok((cont, cmd)) if cmd.as_str() == "rem" => return Ok(Self::Rem(cont)),
            Ok(ok) => ok,
            Err(_) => return Err(ParseError::syntax_error(s, "missing arguments")),
        };
        match command.as_ref() {
            "title" => Ok(Self::Title(trim!(content))),
            "performer" => Ok(Self::Performer(trim!(content))),
            "songwriter" => Ok(Self::Songwriter(trim!(content))),
            "catalog" => match utils::number(13)(content) {
                Ok((_, catalog)) => Ok(Self::Catalog(catalog)),
                Err(_) => return Err(ParseError::syntax_error(content, "invaild catalog"))
            },
            "cdtextfile" => Ok(Self::Cdtextfile(trim!(content))),
            "file" => match utils::quote_opt(content) {
                Ok((format, path)) => Ok(Self::File(trim!(path), format.trim())),
                Err(_) => return Err(ParseError::syntax_error(command, "missing arguments")),
            },
            "track" => match utils::token(content) {
                Ok((format, id)) => Ok(Self::Track(utils::number(2)(id)?.1, format)),
                Err(_) => return Err(ParseError::syntax_error(command, "missing arguments")),
            },
            "index" => match utils::token(content) {
                Ok((timestamp, id)) => Ok(Self::Index(utils::number(2)(id)?.1, timestamp.parse()?)),
                Err(_) => return Err(ParseError::syntax_error(command, "missing arguments")),
            },
            "pregap" => Ok(Self::Pregap(trim!(content))),
            "postgap" => Ok(Self::Postgap(trim!(content))),
            "isrc" => Ok(Self::Isrc(trim!(content))),
            "flag" => Ok(Self::Flags(trim!(content))),
            _ => Err(ParseError::unexpected_token(command)),
        }
    }
}
impl fmt::Display for Command<'_> {
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
            Command::Index(id, timestamp) => format!("INDEX {} {}", id, timestamp),
            Command::Pregap(c) => format!("PREGAP {}", c),
            Command::Postgap(c) => format!("POSTGAP {}", c),
            Command::Isrc(c) => format!("ISRC {}", c),
            Command::Flags(c) => format!("FLAG {}", c),
        };
        write!(formatter, "{}", command)
     }
}
impl<'a> Line<'a> {
    pub fn new(s: &'a str, line: usize) -> Result<Self, Error> {
        let command = Command::new(s).map_err(
            |e| Error::new(e, line)
        )?;
        Ok( Self { command, line })
    }
    pub const fn command(&self) -> &Command {
        &self.command
    }
    pub const fn line(&self) -> usize {
        self.line
    }
    pub fn parse(&self, sheet: &mut CueSheet) -> Result<(), Error> {
        let command = self.command();
        match *command {
            Command::Rem(s) => sheet.comments.push(s.to_owned()),
            Command::Title(s) => match sheet.last_track_mut() {
                Some(tk) => tk.push_title(s.to_owned()),
                None => sheet.header.push_title(s.to_owned()),
            },
            Command::Performer(s) => match sheet.last_track_mut() {
                Some(tk) => tk.push_performer(s.to_owned()),
                _ => sheet.header.push_performer(s.to_owned()),
            },
            Command::Songwriter(s) => match sheet.last_track_mut() {
                Some(tk) => tk.push_songwriter(s.to_owned()),
                _ => sheet.header.push_songwriter(s.to_owned()),
            },
            Command::Catalog(s) => if sheet.header.catalog.is_none() {
                sheet.header.catalog = Some(s);
            } else {
                fail!(syntax self, command, "multiple `CATALOG` commands is not allowed")
            }
            Command::Cdtextfile(s) => {
                sheet.header.set_cdtextfile(s.to_owned());
            },
            Command::File(name, format) => {
                sheet.push_file(TrackInfo::new(name.to_owned(), format.to_owned()));
            },
            Command::Track(id, format) => {
                match sheet.last_file_mut() {
                    Some(tk) => tk.push_track(Track::new_unchecked(id, format.to_owned())),
                    None => fail!(syntax self, command, "Multiple `CATALOG` commands is not allowed")
                }
            },
            Command::Index(id, timestamp) => match sheet.last_track_mut() {
                Some(tk) if tk.postgap.is_none() => {
                    tk.push_index(Index::new_unchecked(id, timestamp))
                },
                Some(_) => fail!(syntax self, command, "Command `INDEX` should be before `POSTGAP`"),
                None => fail!(token self, "INDEX"),
            }
            Command::Pregap(timestamp) => match sheet.last_track_mut() {
                Some(tk) if tk.index.is_empty() && tk.pregap.is_none() => {
                    tk.set_pregep(timestamp.parse()?);
                },
                Some(tk) if !tk.index.is_empty() => fail!(syntax self, command, "Command `PREGAP` should be before `INDEX`"),
                Some(tk) if tk.pregap.is_some() => fail!(syntax self, command, "Multiple `PREGAP` commands are not allowed in one `TRACK` scope"),
                _ => fail!(token self, "PREGAP"),
            },
            Command::Postgap(timestamp) => match sheet.last_track_mut() {
                Some(tk) if tk.postgap.is_none() => {
                    tk.set_postgep(timestamp.parse()?);
                },
                Some(_) => fail!(syntax self, command, "Multiple `POSTGAP` commands are not allowed in one `TRACK` scope"),
                None => fail!(token self, "POSTGAP"),
            },
            Command::Isrc(s) => match sheet.last_track_mut() {
                Some(tk) if tk.isrc.is_none() => {
                    tk.set_isrc(s.to_owned());
                },
                Some(_) => fail!(syntax self, command, "Multiple `ISRC` commands are not allowed in one `TRACK` scope"),
                None => fail!(token self, "ISRC"),
            },
            Command::Flags(s) => match sheet.last_track_mut() {
                Some(tk) if tk.flags.is_none() => tk.push_flags(s.split(' ')),
                Some(_) => fail!(syntax self, command, "Multiple `FLAGS` commands are not allowed in one `TRACK` scope"),
                None => fail!(token self, "FLAGS"),
            }
        }
        Ok(())
    }
}
impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Result<Self, Error> {
        Self::with_state(s, CueSheet::default())
    }
    pub fn with_state(s: &'a str, state: CueSheet) -> Result<Self, Error> {
        let lines = s.lines()
            .enumerate()
            .map(|(line, content)| Line::new(content, line + 1))
            .filter(|r| r != &Err(Error::EMPTY))
            .collect::<Result<_, _>>()?;
        Ok(Self { lines, state })
    }
    pub fn current_line(&self) -> Option<&Line> {
        self.lines.front()
    }
    pub fn state(&self) -> &CueSheet {
        &self.state
    }
    pub fn parse_next_line(&mut self) -> Result<Line<'_>, Error> {
        let current_line = match self.lines.pop_front() {
            Some(cl) => cl,
            None => return Err(Error::EMPTY),
        };
        current_line.parse(&mut self.state)?;
        Ok(current_line)
    }
    pub fn parse(mut self) -> Result<CueSheet, Error> {
        self.parse_to_end().map(|_| self.state)
    }
    pub fn parse_to_end(&mut self) -> Result<(), Error> {
        let sheet = &mut self.state;
        self.lines.iter()
            .map(|l| l.parse(sheet))
            .collect()
    }
}