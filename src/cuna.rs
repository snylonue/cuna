use crate::comment::Comment;
use crate::error::Error;
use crate::header::Header;
use crate::parser::Command;
use crate::parser::Parna;
use crate::track::Track;
use crate::track::Disc;
use crate::trim_utf8_header;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Cursor;
use std::io::Read;
use std::iter::Flatten;
use std::ops::Index;
use std::path::Path;
use std::slice::Iter;
use std::str::FromStr;

/// Represents a cue sheet
///
/// All constructors internally use [`Parna`](crate::parser::Parna) as a parser
/// and stops parsing as long as an error occured.
///
/// If you want to ignore some errors, try [`Parna::parse()`](crate::parser::Parna::parse) to manually deal with them.
///
/// In most cases, you just need to keep calling it until an `Ok(())` is returned.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Cuna {
    pub header: Header,
    pub files: Vec<Disc>,
    pub comments: Comment,
}

impl Cuna {
    /// Parses an str as cue sheet
    /// ```rust
    /// use cuna::Cuna;
    /// let sheet = Cuna::new("REM a cue sheet").unwrap();
    /// ```
    #[inline(always)]
    pub fn new(s: &str) -> Result<Self, Error> {
        s.parse()
    }
    /// Parses an str as cue sheet like [`new()`](Self::new) but never fails
    /// by skipping bad lines
    pub fn new_suc(s: &str) -> Self {
        let cursor = Cursor::new(s);
        // Never panics since `Cursor::fill_buf()` always returns `Ok()`
        Self::from_buf_read_suc(cursor).unwrap()
    }
    pub const fn with_parts(header: Header, files: Vec<Disc>, comments: Comment) -> Self {
        Self {
            header,
            files,
            comments,
        }
    }
    /// Reads `f` into a string and parses it.
    pub fn read(mut f: impl Read) -> Result<Self, Error> {
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;
        buf.parse()
    }
    /// Like [`Cuna::read()`](Self::read) but skips bad lines
    pub fn read_suc(mut f: impl Read) -> Result<Self, Error> {
        let mut buf = String::new();
        f.read_to_string(&mut buf)?;
        Ok(Self::new_suc(&buf))
    }
    /// Parses a file as a cue sheet
    ///
    /// **Only supports UTF-8 encoding (with BOM or not)**
    pub fn from_file(file: &File) -> Result<Self, Error> {
        let buffer = BufReader::new(file);
        Self::from_buf_read(buffer)
    }
    /// Opens a file and parses it as a cue sheet
    ///
    /// **Only supports UTF-8 encoding (with BOM or not)**
    ///
    /// ```rust
    /// use cuna::Cuna;
    /// use cuna::error::Error;
    ///
    /// let file = "tests/EGOIST - Departures ～あなたにおくるアイの歌～.cue";
    /// let cue = Cuna::open(file).unwrap();
    /// assert_eq!(cue.comments[0], "GENRE Pop");
    /// assert_eq!(cue.title(), &["Departures ～あなたにおくるアイの歌～".to_owned()]);
    /// assert_eq!(cue[0].name, "EGOIST - Departures ～あなたにおくるアイの歌～.flac");
    /// assert_eq!(cue[0][0].performer(), &["EGOIST".to_owned()]);
    /// ```
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file = File::open(path)?;
        Self::from_file(&file)
    }
    /// Opens a file and parses it as a cue sheet like [`open()`](Self::open()),
    /// except this method skips bad lines.
    ///
    /// **Only supports UTF-8 encoding (with BOM or not)**
    pub fn open_suc<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let buffer = BufReader::new(File::open(path)?);
        Self::from_buf_read_suc(buffer)
    }
    /// Reads `buf` and parses it as a cue sheet
    ///
    /// **Only supports UTF-8 encoding (with BOM or not)**
    pub fn from_buf_read(mut buf: impl BufRead) -> Result<Self, Error> {
        let mut sheet = Self::default();
        let mut buffer = String::new();
        let mut at = 1;
        loop {
            match buf.read_line(&mut buffer) {
                Ok(0) => break Ok(sheet),
                Ok(_) => {
                    let err = |e| Error::new(e, at);
                    let command = Command::new(trim_utf8_header(&buffer)).map_err(err)?;
                    command.parse(&mut sheet).map_err(err)?;
                }
                Err(e) => break Err(Error::new(e.into(), at)),
            }
            at += 1;
            buffer.clear();
        }
    }
    /// Reads `buf` and parses it as a cue sheet like [`from_buf_read()`](Self::from_buf_read()),
    /// except this method skips bad lines.
    ///
    /// **Only supports UTF-8 encoding (with BOM or not)**
    pub fn from_buf_read_suc(mut buf: impl BufRead) -> std::io::Result<Self> {
        let mut sheet = Self::default();
        let mut buffer = String::new();
        loop {
            match buf.read_line(&mut buffer) {
                Ok(0) => break Ok(sheet),
                Ok(_) => {
                    let command = match Command::new(trim_utf8_header(&buffer)) {
                        Ok(c) => c,
                        _ => continue,
                    };
                    let _ = command.parse(&mut sheet);
                }
                Err(e) => break Err(e),
            }
            buffer.clear();
        }
    }
    pub fn header(&self) -> &Header {
        &self.header
    }
    pub fn title(&self) -> &Vec<String> {
        self.header.title()
    }
    pub fn performer(&self) -> &Vec<String> {
        self.header.performer()
    }
    pub fn songwriter(&self) -> &Vec<String> {
        self.header.songwriter()
    }
    pub fn catalog(&self) -> Option<u64> {
        self.header.catalog()
    }
    pub fn files(&self) -> &Vec<Disc> {
        &self.files
    }
    pub fn set_files(&mut self, files: Vec<Disc>) -> Vec<Disc> {
        std::mem::replace(&mut self.files, files)
    }
    pub fn comments(&self) -> &Comment {
        &self.comments
    }
    pub fn push_file(&mut self, track: Disc) {
        self.files.push(track);
    }
    /// Returns the first, usually the only `FILE` field in the cue sheet
    pub fn first_file(&self) -> Option<&Disc> {
        self.files.first()
    }
    /// The mutable version of [`Cuna::first_file()`](Cuna::first_file)
    pub fn first_file_mut(&mut self) -> Option<&mut Disc> {
        self.files.first_mut()
    }
    /// Returns the last, usually the only `FILE` field in the cue sheet
    pub fn last_file(&self) -> Option<&Disc> {
        self.files.last()
    }
    /// The mutable version of [`Cuna::last_file()`](Cuna::last_file)
    pub fn last_file_mut(&mut self) -> Option<&mut Disc> {
        self.files.last_mut()
    }
    /// Returns the last `TRACK` field which appears in the cue sheet
    pub fn last_track(&self) -> Option<&Track> {
        self.last_file().and_then(Disc::last_track)
    }
    /// The mutable version of [`Cuna::last_track()`](Cuna::last_track)
    pub fn last_track_mut(&mut self) -> Option<&mut Track> {
        self.last_file_mut().and_then(Disc::last_track_mut)
    }
    /// An iterator over the `TRACK`s in all the `FILE`s
    pub fn tracks(&self) -> Flatten<Iter<Disc>> {
        self.files.iter().flatten()
    }
}
impl FromStr for Cuna {
    type Err = Error;

    /// Parses an str as cue sheet
    /// ```rust
    /// use std::str::FromStr;
    /// use cuna::Cuna;
    /// let sheet = Cuna::from_str("REM a cue sheet").unwrap();
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sheet = Cuna::default();
        Parna::new(crate::trim_utf8_header(s)).parse(&mut sheet)?;
        Ok(sheet)
    }
}
impl Index<usize> for Cuna {
    type Output = Disc;

    /// # Panics
    ///
    /// panics if index out of range
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.files[index]
    }
}
