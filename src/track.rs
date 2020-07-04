use nom::sequence::tuple;
use nom::sequence::delimited;
use nom::bytes::complete::tag_no_case as tag;
use nom::combinator::rest;
use nom::combinator::map_res;
use nom::combinator::map;
use std::str::FromStr;
use crate::time::Duration;
use crate::utils;
use crate::error::ParseError;

#[derive(Debug, Clone)]
pub struct Index {
    pub(crate) id: u8, // index id must between 1 and 99
    pub begin_time: Duration,
}
#[derive(Debug, Clone, Default)]
pub struct Track {
    pub(crate) id: u8, // track-id must between 1 and 99
    pub format: String,
    pub index: Vec<Index>,
    pub pregap: Option<Duration>,
    pub postgap: Option<Duration>,
    pub title: Option<Vec<String>>,
    pub performer: Option<Vec<String>>,
    pub songwriter: Option<Vec<String>>,
    pub isrc: Option<String>,
    pub flags: Option<Vec<String>>
}
#[derive(Debug, Clone)]
pub struct TrackInfo {
    pub name: String,
    pub format: String,
    pub tracks: Vec<Track>,
}

impl Index {
    pub(crate) const fn new_unchecked(id: u8, begin_time: Duration) -> Self {
        Self { id, begin_time }
    }
    pub fn new(id: u8, begin_time: Duration) -> Self {
        Self::new_opt(id, begin_time).expect("index-id must be between 1 and 99")
    }
    pub fn new_opt(id: u8, begin_time: Duration) -> Option<Self> {
        if id <= 99 {
            Some(Self::new_unchecked(id, begin_time))
        } else {
            None
        }
    }
}
impl FromStr for Index {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, index) = map(
            tuple((
                delimited(utils::keyword("INDEX"), utils::number(2), tag(" ")),
                map_res(rest, Duration::from_str)
            )),
            |(id, begin_time)| Self::new_unchecked(id, begin_time)
        )(s)?;
        Ok(index)
    }
}
impl Track {
    pub(crate) fn new_unchecked(id: u8, format: String) -> Self {
        Self { id, format, ..Self::default() }
    }
    /// Constructs a new Track
    /// 
    /// # Panics
    ///
    /// Panics if id > 99
    pub fn new(id: u8, format: String) -> Self {
        Self::new_opt(id, format).expect("track-id must be between 1 and 99")
    }
    pub fn new_opt(id: u8, format: String) -> Option<Self> {
        if id <= 99 {
            Some(Self::new_unchecked(id, format))
        } else {
            None
        }
    }
    pub fn push_title(&mut self, title: String) {
        self.title.get_or_insert_with(|| Vec::with_capacity(1)).push(title)
    }
    pub fn push_performer(&mut self, performer: String) {
        self.performer.get_or_insert_with(|| Vec::with_capacity(1)).push(performer)
    }
    pub fn push_songwriter(&mut self, songwriter: String) {
        self.songwriter.get_or_insert_with(|| Vec::with_capacity(1)).push(songwriter)
    }
    pub fn push_index(&mut self, index: Index) {
        self.index.push(index)
    }
    pub fn set_pregep(&mut self, pregap: Duration) -> Option<Duration> {
        self.pregap.replace(pregap)
    }
    pub fn set_postgep(&mut self, postgap: Duration) -> Option<Duration> {
        self.postgap.replace(postgap)
    }
    pub fn set_isrc(&mut self, isrc: String) -> Option<String> {
        self.isrc.replace(isrc)
    }
    pub fn push_flag(&mut self, flag: String) {
        self.flags.get_or_insert_with(|| Vec::with_capacity(1)).push(flag)
    }
    pub fn push_flags<F, S>(&mut self, flags: F)
        where F: IntoIterator<Item = S>,
            S: Into<String>
    {
        self.flags.get_or_insert_with(|| Vec::new()).extend(flags.into_iter().map(Into::into))
    }
}
impl FromStr for Track {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (tp, id) = delimited(utils::keyword("TRACK"), utils::number(2), tag(" "))(s)?;
        Ok(Self::new_unchecked(id, tp.to_owned()))
    }
}
impl TrackInfo {
    /// Constructs a new TrackInfo
    pub const fn new(name: String, format: String) -> Self {
        Self::with_tracks(name, format, Vec::new())
    }
    pub const fn with_tracks(name: String, format: String, tracks: Vec<Track>) -> Self {
        Self { name, format, tracks }
    }
    /// Returns the last Track or None if self.tracks is empty
    pub fn last_track(&self) -> Option<&Track> {
        self.tracks.last()
    }
    /// The mutable version of last_track()
    pub fn last_track_mut(&mut self) -> Option<&mut Track> {
        self.tracks.last_mut()
    }
    /// Appends an element to the back of self.tracks
    pub fn push_track(&mut self , track: Track) {
        self.tracks.push(track)
    }
}