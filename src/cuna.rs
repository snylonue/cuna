use std::str::FromStr;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::Index;
use std::iter::Flatten;
use std::slice::Iter;
use crate::track::Track;
use crate::track::TrackInfo;
use crate::header::Header;
use crate::comment::Comment;
use crate::error::Error;
use crate::parser::Parser;

/// Represents a cue sheet
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Cuna {
    pub header: Header,
    pub files: Vec<TrackInfo>,
    pub comments: Comment,
}

impl Cuna {
    pub const fn new(header: Header, files: Vec<TrackInfo>, comments: Comment) -> Self {
        Self { header, files, comments }
    }
    pub fn from_utf8_with_bom(s: &str) -> Result<Self, Error> {
        Ok(crate::trim_utf8_header(s).parse()?) // remove UTF-8 BOM header
    }
    /// Parses a file as a cue sheet
    /// 
    /// **File must use UTF-8 encoding (BOM header will be removed)**
    ///
    /// ```rust
    /// use cuna::Cuna;
    /// use cuna::error::Error;
    ///
    /// let file = "tests/EGOIST - Departures ～あなたにおくるアイの歌～.cue";
    /// let cue = Cuna::open(file).unwrap();
    /// assert_eq!(cue.comments[0], "GENRE Pop");
    /// assert_eq!(cue.header.title, Some(vec!["Departures ～あなたにおくるアイの歌～".to_owned()]));
    /// assert_eq!(cue[0].name, "EGOIST - Departures ～あなたにおくるアイの歌～.flac");
    /// assert_eq!(cue.last_track().unwrap().performer(), Some(&vec!["EGOIST".to_owned()]));
    /// ```
    pub fn from_file(file: &mut File) -> Result<Self, Error> {
        let mut buffer = BufReader::new(file);
        Self::from_buf_read(&mut buffer)
    }
    /// Opens a file and parses it as a cue sheet
    /// 
    /// **File must use UTF-8 encoding (BOM header will be removed)**
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let mut file = File::open(path)?;
        Self::from_file(&mut file)
    }
    pub fn from_buf_read(buf: &mut impl BufRead) -> Result<Self, Error> {
        let mut sheet = Self::default();
        for (at, line) in buf.lines().enumerate() {
            let line = line?;
            Parser::new(crate::trim_utf8_header(&line))
                .parse(&mut sheet)
                .map_err(|mut e| {
                    e.set_pos(at + 1);
                    e
                })?;
        }
        Ok(sheet)
    }
    pub fn header(&self) -> &Header {
        &self.header
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
    pub fn first_file(&self) -> Option<&TrackInfo> {
        self.files.first()
    }
    pub fn first_file_mut(&mut self) -> Option<&mut TrackInfo> {
        self.files.first_mut()
    }
    pub fn last_file(&self) -> Option<&TrackInfo> {
        self.files.last()
    }
    pub fn last_file_mut(&mut self) -> Option<&mut TrackInfo> {
        self.files.last_mut()
    }
    pub fn last_track(&self) -> Option<&Track> {
        self.last_file().map(|tk| tk.last_track()).flatten()
    }
    pub fn last_track_mut(&mut self) -> Option<&mut Track> {
        self.last_file_mut().map(|tk| tk.last_track_mut()).flatten()
    }
    pub fn tracks(&self) -> Flatten<Iter<TrackInfo>> {
        self.files.iter().flatten()
    }
}
impl FromStr for Cuna {
    type Err = Error;

    /// s must be UTF-8 encoding without BOM header
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sheet = Cuna::default();
        Parser::new(s).parse(&mut sheet)?;
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