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
    pub tracks: Vec<TrackInfo>,
    pub comments: Comment,
}

impl CueSheet {
    pub const fn new(header: Header, tracks: Vec<TrackInfo>, comments: Comment) -> Self {
        Self { header, tracks, comments }
    }
    /// Parses a file as a cue sheet
    /// 
    /// **File must use UTF-8 encoding (BOM header will be removed)**
    pub fn from_file(file: &mut File) -> Result<Self, Error> {
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        Ok(buf.trim_start_matches('﻿').parse()?) // remove UTF-8 BOM header
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
    pub fn tracks(&self) -> &Vec<TrackInfo> {
        &self.tracks
    }
    pub fn comments(&self) -> &Comment {
        &self.comments
    }
    pub fn push_track_info(&mut self, track: TrackInfo) {
        self.tracks.push(track);
    }
    pub fn last_track_info(&self) -> Option<&TrackInfo> {
        self.tracks.last()
    }
    pub fn last_track_info_mut(&mut self) -> Option<&mut TrackInfo> {
        self.tracks.last_mut()
    }
    pub fn last_track(&self) -> Option<&Track> {
        self.last_track_info().map(|tk| tk.last_track()).flatten()
    }
    pub fn last_track_mut(&mut self) -> Option<&mut Track> {
        self.last_track_info_mut().map(|tk| tk.last_track_mut()).flatten()
    }
}
impl FromStr for CueSheet {
    type Err = Error;

    /// s must be UTF-8 encoding without BOM header
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::Parser::new(s)?.parse()
    }
}