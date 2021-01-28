use crate::comment::Comment;
use crate::error::Error;
use crate::header::Header;
use crate::parser::Command;
use crate::parser::Parser;
use crate::track::Track;
use crate::track::TrackInfo;
use crate::trim_utf8_header;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::Flatten;
use std::ops::Index;
use std::path::Path;
use std::slice::Iter;
use std::str::FromStr;

/// Represents a cue sheet
///
/// See [`parser::Parser`](crate::parser::Parser) to deal with errors when parsing
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Cuna {
    pub header: Header,
    pub files: Vec<TrackInfo>,
    pub comments: Comment,
}

impl Cuna {
    #[deprecated = "This method will be changed soon, use Cuna::with_parts() instead"]
    pub const fn new(header: Header, files: Vec<TrackInfo>, comments: Comment) -> Self {
        Self {
            header,
            files,
            comments,
        }
    }
    pub const fn with_parts(header: Header, files: Vec<TrackInfo>, comments: Comment) -> Self {
        Self {
            header,
            files,
            comments,
        }
    }
    #[deprecated = "use Cuna::from_str() instead"]
    pub fn from_utf8_with_bom(s: &str) -> Result<Self, Error> {
        Ok(crate::trim_utf8_header(s).parse()?) // remove UTF-8 BOM header
    }
    /// Parses a file as a cue sheet
    ///
    /// **Only UTF-8 encoding is supported (BOM header will be removed)**
    pub fn from_file(file: &mut File) -> Result<Self, Error> {
        let mut buffer = BufReader::new(file);
        Self::from_buf_read(&mut buffer)
    }
    /// Opens a file and parses it as a cue sheet
    ///
    /// **Only UTF-8 encoding is supported (BOM will be removed)**
    ///
    /// ```rust
    /// use cuna::Cuna;
    /// use cuna::error::Error;
    ///
    /// let file = "tests/EGOIST - Departures ～あなたにおくるアイの歌～.cue";
    /// let cue = Cuna::open(file).unwrap();
    /// assert_eq!(cue.comments[0], "GENRE Pop");
    /// assert_eq!(cue.title(), &vec!["Departures ～あなたにおくるアイの歌～".to_owned()]);
    /// assert_eq!(cue[0].name, "EGOIST - Departures ～あなたにおくるアイの歌～.flac");
    /// assert_eq!(cue[0][0].performer(), &vec!["EGOIST".to_owned()]);
    /// ```
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let mut file = File::open(path)?;
        Self::from_file(&mut file)
    }
    pub fn from_buf_read(buf: &mut impl BufRead) -> Result<Self, Error> {
        let mut sheet = Self::default();
        let mut buffer = String::new();
        let mut at = 1;
        loop {
            match buf.read_line(&mut buffer) {
                Ok(0) => break Ok(sheet),
                Ok(_) => {
                    let command =
                        Command::new(trim_utf8_header(&buffer)).map_err(|e| Error::new(e, at))?;
                    command.parse(&mut sheet).map_err(|e| Error::new(e, at))?;
                }
                Err(e) => break Err(Error::new(e.into(), at)),
            }
            at += 1;
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
        self.header.songwriter().as_ref()
    }
    pub fn catalog(&self) -> Option<u64> {
        self.header.catalog()
    }
    pub fn files(&self) -> &Vec<TrackInfo> {
        &self.files
    }
    pub fn set_files(&mut self, files: Vec<TrackInfo>) -> Vec<TrackInfo> {
        std::mem::replace(&mut self.files, files)
    }
    pub fn comments(&self) -> &Comment {
        &self.comments
    }
    pub fn push_file(&mut self, track: TrackInfo) {
        self.files.push(track);
    }
    /// Returns the first, usually the only `FILE` field in the cue sheet
    pub fn first_file(&self) -> Option<&TrackInfo> {
        self.files.first()
    }
    /// The mutable version of [`Cuna::first_file()`](Cuna::first_file)
    pub fn first_file_mut(&mut self) -> Option<&mut TrackInfo> {
        self.files.first_mut()
    }
    /// Returns the last, usually the only `FILE` field in the cue sheet
    pub fn last_file(&self) -> Option<&TrackInfo> {
        self.files.last()
    }
    /// The mutable version of [`Cuna::last_file()`](Cuna::last_file)
    pub fn last_file_mut(&mut self) -> Option<&mut TrackInfo> {
        self.files.last_mut()
    }
    /// Returns the last `TRACK` field which appears in the cue sheet
    pub fn last_track(&self) -> Option<&Track> {
        self.last_file().map(|tk| tk.last_track()).flatten()
    }
    /// The mutable version of [`Cuna::last_track()`](Cuna::last_track)
    pub fn last_track_mut(&mut self) -> Option<&mut Track> {
        self.last_file_mut().map(|tk| tk.last_track_mut()).flatten()
    }
    /// An iterator over the `TRACK`s in all the `FILE`s
    pub fn tracks(&self) -> Flatten<Iter<TrackInfo>> {
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
        Parser::new(crate::trim_utf8_header(s)).parse(&mut sheet)?;
        Ok(sheet)
    }
}
impl Index<usize> for Cuna {
    type Output = TrackInfo;

    /// # Panics
    ///
    /// panics if index out of range
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.files[index]
    }
}
