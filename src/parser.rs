use anyhow::Result;
use std::collections::VecDeque;
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
    len: usize,
    sheet: CueSheet,
}

impl<'a> Command<'a> {
    pub fn new(s: &'a str) -> Result<Self> {
        let (content, command) = utils::split_space(s)
            .map_err(|_| anyhow::anyhow!("Invaild command {}", s))?;
        let (rest, content) = utils::quote_opt(content.trim())
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
                let (format, id) = utils::split_space(content)
                    .map_err(|_| anyhow::anyhow!("Invaild command {}", content))?;
                Ok(Self::Track(id, format))
            },
            "index" => {
                let (duration, id) = utils::split_space(content)
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
}
impl<'a> Line<'a> {
    pub fn new(s: &'a str, line: usize) -> Result<Self> {
        let indentations = utils::indentation_count(s);
        let command = Command::new(&s.trim()).map_err(|e| anyhow::anyhow!("{} at line {}", e, line + 1))?;
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
        let len = lines.len();
        Ok(Self { lines, len, sheet: CueSheet::default() })
    }
    pub fn current_line(&self) -> Option<&Line> {
        self.lines.front()
    }
    pub fn parse_next_line(&mut self) -> Result<()> {
        let current_line = self.lines.pop_front().unwrap();
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
        Ok(())
    }
    pub fn parse(mut self) -> Result<CueSheet> {
        while !self.lines.is_empty() {
            match self.parse_next_line() {
                Ok(_) => {},
                Err(e) => anyhow::bail!("Error at line {}: {}", self.current_line().map(|l| l.line - 1).unwrap_or(self.len), e),
            }
        }
        Ok(self.sheet)
    }
}