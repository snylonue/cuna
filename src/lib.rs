pub mod utils;
pub mod error;
pub mod time;
pub mod header;
pub mod track;
pub mod comment;
pub mod parser;

use std::str::FromStr;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::track::Track;
use crate::track::TrackInfo;
use crate::header::Header;
use crate::comment::Comment;
use crate::error::Error;

/// Represents a cue sheet
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct CueSheet {
    pub header: Header,
    pub files: Vec<TrackInfo>,
    pub comments: Comment,
}

impl CueSheet {
    pub const fn new(header: Header, files: Vec<TrackInfo>, comments: Comment) -> Self {
        Self { header, files, comments }
    }
    pub fn from_utf8_with_bom(s: &str) -> Result<Self, Error> {
        Ok(trim_utf8_header(s).parse()?) // remove UTF-8 BOM header
    }
    /// Parses a file as a cue sheet
    /// 
    /// **File must use UTF-8 encoding (BOM header will be removed)**
    pub fn from_file(file: &mut File) -> Result<Self, Error> {
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        Self::from_utf8_with_bom(&buf)
    }
    /// Opens a file and parses it as a cue sheet
    /// 
    /// **File must use UTF-8 encoding (BOM header will be removed)**
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let mut file = File::open(path)?;
        Self::from_file(&mut file)
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
}
impl FromStr for CueSheet {
    type Err = Error;

    /// s must be UTF-8 encoding without BOM header
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sheet = CueSheet::default();
        parser::Parser::new(s).parse(&mut sheet)?;
        Ok(sheet)
    }
}

#[inline]
pub fn trim_utf8_header(s: &str) -> &str {
    s.trim_start_matches('﻿')
}