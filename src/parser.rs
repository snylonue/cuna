use crate::error::Error;
use crate::error::InvalidArgument;
use crate::error::ParseError;
use crate::time::TimeStamp;
use crate::track::Index;
use crate::track::Track;
use crate::track::TrackInfo;
use crate::utils;
use crate::Cuna;
use std::fmt;
use std::iter::Enumerate;
use std::str::Lines;

pub type Parser<'a> = Parna<Enumerate<Lines<'a>>>;

macro_rules! fail {
    (token $token: expr) => {
        return Err($crate::error::ParseError::unexpected_token($token))
    };
    (syntax $cmd: expr, $msg: expr) => {
        return Err($crate::error::ParseError::syntax_error($cmd, $msg))
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
    Empty,
}

/// A lazy parser takes iterators of `(usize, &str)`
/// which won't parse anything unless `Parna::parse*()` is called
///
/// In most cases, it is constructed with an str using [`Parna::new()`](Parna::new)
///
/// It only stores the original data
/// and results will be written to `Cuna` which is passed to `Parna::parse*()`
#[derive(Debug, Clone)]
pub struct Parna<I>(I);

impl<'a> Command<'a> {
    pub fn new(s: &'a str) -> Result<Self, ParseError> {
        let s = match s.trim() {
            "" => return Ok(Self::Empty),
            ts => ts,
        };
        let (content, command) = match utils::token(s) {
            Ok(ok) => ok,
            Err(_) => fail!(syntax s, "missing arguments"),
        };
        match command.to_ascii_lowercase().as_ref() {
            "rem" => Ok(Self::Rem(content)),
            "title" => Ok(Self::Title(trimq(content))),
            "performer" => Ok(Self::Performer(trimq(content))),
            "songwriter" => Ok(Self::Songwriter(trimq(content))),
            "catalog" => match utils::number(13)(content) {
                Ok((_, catalog)) => Ok(Self::Catalog(catalog)),
                Err(_) => fail!(syntax content, "invaild catalog"),
            },
            "cdtextfile" => Ok(Self::Cdtextfile(trimq(content))),
            "file" => match utils::quote_opt(content) {
                Ok(("", _)) | Err(_) => Err(InvalidArgument::MissingArgument.into()),
                Ok((format, path)) => Ok(Self::File(trimq(path), format.trim())),
            },
            "track" => match utils::token(content) {
                Ok((format, id)) => Ok(Self::Track(parse_id(id)?, format)),
                Err(_) => Err(InvalidArgument::MissingArgument.into()),
            },
            "index" => match utils::token(content) {
                Ok((timestamp, id)) => Ok(Self::Index(parse_id(id)?, timestamp.parse()?)),
                Err(_) => Err(InvalidArgument::MissingArgument.into()),
            },
            "pregap" => Ok(Self::Pregap(trimq(content))),
            "postgap" => Ok(Self::Postgap(trimq(content))),
            "isrc" => Ok(Self::Isrc(trimq(content))),
            "flags" => Ok(Self::Flags(trimq(content))),
            _ => Err(ParseError::unexpected_token(command)),
        }
    }
    pub fn parse(&self, sheet: &mut Cuna) -> Result<(), ParseError> {
        match *self {
            Self::Empty => {}
            Self::Rem(s) => sheet.comments.push(s.to_owned()),
            Self::Title(s) => match sheet.last_track_mut() {
                Some(tk) => tk.push_title(s.to_owned()),
                None => sheet.header.push_title(s.to_owned()),
            },
            Self::Performer(s) => match sheet.last_track_mut() {
                Some(tk) => tk.push_performer(s.to_owned()),
                _ => sheet.header.push_performer(s.to_owned()),
            },
            Self::Songwriter(s) => match sheet.last_track_mut() {
                Some(tk) => tk.push_songwriter(s.to_owned()),
                _ => sheet.header.push_songwriter(s.to_owned()),
            },
            Self::Catalog(s) => match sheet.header.catalog {
                None => sheet.header.catalog = Some(s),
                _ => fail!(syntax self, "multiple `CATALOG` commands is not allowed"),
            },
            Self::Cdtextfile(s) => {
                sheet.header.set_cdtextfile(s.to_owned());
            }
            Self::File(name, format) => {
                sheet.push_file(TrackInfo::new(name.to_owned(), format.to_owned()));
            }
            Self::Track(id, format) => match sheet.last_file_mut() {
                Some(tk) => tk.push_track(Track::new_unchecked(id, format.to_owned())),
                None => fail!(token "TRACK"),
            },
            Self::Index(id, timestamp) => match sheet.last_track_mut() {
                Some(tk) if tk.postgap.is_none() => {
                    tk.push_index(Index::new_unchecked(id, timestamp))
                }
                Some(_) => fail!(syntax self, "Command `INDEX` should be before `POSTGAP`"),
                None => fail!(token "INDEX"),
            },
            Self::Pregap(timestamp) => match sheet.last_track_mut() {
                Some(tk) if tk.index.is_empty() && tk.pregap.is_none() => {
                    tk.set_pregep(timestamp.parse()?);
                }
                Some(tk) if tk.pregap.is_some() => {
                    fail!(syntax self, "Multiple `PREGAP` commands are not allowed in one `TRACK` scope")
                }
                Some(_) => fail!(syntax self, "Command `PREGAP` should be before `INDEX`"),
                _ => fail!(token "PREGAP"),
            },
            Self::Postgap(timestamp) => match sheet.last_track_mut() {
                Some(tk) if tk.postgap.is_none() => {
                    tk.set_postgep(timestamp.parse()?);
                }
                Some(_) => {
                    fail!(syntax self, "Multiple `POSTGAP` commands are not allowed in one `TRACK` scope")
                }
                None => fail!(token "POSTGAP"),
            },
            Self::Isrc(s) => match sheet.last_track_mut() {
                Some(tk) if tk.isrc.is_none() => {
                    tk.set_isrc(s.to_owned());
                }
                Some(_) => {
                    fail!(syntax self, "Multiple `ISRC` commands are not allowed in one `TRACK` scope")
                }
                None => fail!(token "ISRC"),
            },
            Self::Flags(s) => match sheet.last_track_mut() {
                Some(tk) if tk.flags.is_empty() => tk.push_flags(s.split(' ')),
                Some(_) => {
                    fail!(syntax self, "Multiple `FLAGS` commands are not allowed in one `TRACK` scope")
                }
                None => fail!(token "FLAGS"),
            },
        }
        Ok(())
    }
}
impl fmt::Display for Command<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Rem(c) => write!(formatter, "REM {}", c),
            Self::Title(c) => write!(formatter, r#"TITLE "{}""#, c),
            Self::Performer(c) => write!(formatter, r#"PERFORMER "{}""#, c),
            Self::Songwriter(c) => write!(formatter, r#"SONGWRITER "{}""#, c),
            Self::Catalog(c) => write!(formatter, "CATALOG {}", c),
            Self::Cdtextfile(c) => write!(formatter, r#"CDTEXTFILE "{}""#, c),
            Self::File(name, tp) => write!(formatter, r#"FILE "{}" {}"#, name, tp),
            Self::Track(id, format) => write!(formatter, "TRACK {} {}", id, format),
            Self::Index(id, timestamp) => write!(formatter, "INDEX {} {}", id, timestamp),
            Self::Pregap(c) => write!(formatter, "PREGAP {}", c),
            Self::Postgap(c) => write!(formatter, "POSTGAP {}", c),
            Self::Isrc(c) => write!(formatter, "ISRC {}", c),
            Self::Flags(c) => write!(formatter, "FLAG {}", c),
            Self::Empty => Ok(()),
        }
    }
}
impl<'a> Parna<Enumerate<Lines<'a>>> {
    /// Returns a new Parser
    pub fn new(s: &'a str) -> Self {
        Self(s.lines().enumerate())
    }
}
impl<'a, I: Iterator<Item = &'a str>> Parna<Enumerate<I>> {
    pub fn from_lines(lines: I) -> Self {
        Self(lines.enumerate())
    }
    #[deprecated]
    pub fn set_lines(&mut self, lines: I) {
        self.0 = lines.enumerate();
    }
}
impl<'a, I: Iterator<Item = (usize, &'a str)>> Parna<I> {
    /// Constructs a `Parna` with a generic iterator of `(usize, &str)`
    ///
    /// The `usize` represents which line is being parsed
    /// and the `&str` represents the actual data
    pub fn with_iter(it: I) -> Self {
        Self(it)
    }
    /// Returns a mut reference to the internal iterator
    pub fn data(&mut self) -> &mut I {
        self.0.by_ref()
    }
    /// Parses one line and writes to `state`
    pub fn parse_next_line(&mut self, state: &mut Cuna) -> Result<(), Error> {
        self.parse_next_n_lines(1, state)
    }
    /// Parses n lines and writes to `state`
    ///
    /// Each line will be parsed and written to `state` until an `Error` is returned
    pub fn parse_next_n_lines(&mut self, n: usize, state: &mut Cuna) -> Result<(), Error> {
        for (at, line) in self.0.by_ref().take(n) {
            let to_error = |e| Error::new(e, at + 1);
            Command::new(line)
                .map_err(to_error)?
                .parse(state)
                .map_err(to_error)?;
        }
        Ok(())
    }
    /// Parses all the lines and writes to `state`
    ///
    /// Each line will be parsed and written to `state` until an `Error` is returned
    ///
    /// If all the lines are parsed successfully, an `Ok(())` will be returned
    pub fn parse(&mut self, state: &mut Cuna) -> Result<(), Error> {
        for (at, line) in self.0.by_ref() {
            let to_error = |e| Error::new(e, at + 1);
            Command::new(line)
                .map_err(to_error)?
                .parse(state)
                .map_err(to_error)?;
        }
        Ok(())
    }
}
impl<'a, I: Iterator<Item = (usize, &'a str)> + Clone> Parna<I> {
    /// Returns the current line to be parsed
    /// ```rust
    /// use cuna::parser::Parser;
    /// let line = r#"TITLE "HELLO WORLD オリジナル・サウンドトラック""#;
    /// let parser = Parser::new(line);
    /// assert_eq!(parser.current_line(), Some(line));
    /// ```
    pub fn current_line(&self) -> Option<&'a str> {
        self.current().map(|(_, s)| s)
    }
    /// Like [`current_line()`](Parna::current_line), but returns line number at the same time
    pub fn current(&self) -> Option<(usize, &'a str)> {
        self.0.clone().next()
    }
}

#[inline(always)]
fn parse_id(s: &str) -> Result<u8, InvalidArgument> {
    Ok(utils::number(2)(s)
        .map_err(|_| InvalidArgument::InvalidId)?
        .1)
}
#[inline(always)]
fn trimq(s: &str) -> &str {
    s.trim_matches('"')
}
