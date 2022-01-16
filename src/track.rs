use crate::error::InvalidArgument;
use crate::time::TimeStamp;
use crate::utils;
use nom::bytes::complete::tag_no_case as tag;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::rest;
use nom::sequence::delimited;
use nom::sequence::tuple;
use std::ops;
use std::str::FromStr;

pub use self::TrackInfo as File;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Copy)]
pub struct Index {
    pub(crate) id: u8, // index id must between 1 and 99
    pub begin_time: TimeStamp,
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Track {
    pub(crate) id: u8, // track-id must between 1 and 99
    pub format: String,
    pub index: Vec<Index>,
    pub pregap: Option<TimeStamp>,
    pub postgap: Option<TimeStamp>,
    pub title: Vec<String>,
    pub performer: Vec<String>,
    pub songwriter: Vec<String>,
    pub isrc: Option<String>,
    pub flags: Vec<String>,
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TrackInfo {
    pub name: String,
    pub format: String,
    pub tracks: Vec<Track>,
}

impl Index {
    pub(crate) const fn new_unchecked(id: u8, begin_time: TimeStamp) -> Self {
        Self { id, begin_time }
    }
    pub fn new(id: u8, begin_time: TimeStamp) -> Self {
        Self::new_opt(id, begin_time).expect("index-id must be between 1 and 99")
    }
    pub fn new_opt(id: u8, begin_time: TimeStamp) -> Option<Self> {
        if id <= 99 {
            Some(Self::new_unchecked(id, begin_time))
        } else {
            None
        }
    }
    pub fn id(&self) -> u8 {
        self.id
    }
    pub fn begin_time(&self) -> &TimeStamp {
        &self.begin_time
    }
}
impl FromStr for Index {
    type Err = InvalidArgument;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, index) = map(
            tuple((
                delimited(utils::keyword("INDEX"), utils::number(2), tag(" ")),
                map_res(rest, TimeStamp::from_str),
            )),
            |(id, begin_time)| Self::new_unchecked(id, begin_time),
        )(s)
        .map_err(|_| InvalidArgument::InvalidId)?;
        Ok(index)
    }
}
impl Track {
    pub(crate) fn new_unchecked(id: u8, format: String) -> Self {
        Self {
            id,
            format,
            ..Self::default()
        }
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
    pub fn id(&self) -> u8 {
        self.id
    }
    pub fn format(&self) -> &str {
        &self.format
    }
    pub fn index(&self) -> &Vec<Index> {
        &self.index
    }
    /// Searches for index by id
    pub fn get_index(&self, id: u8) -> Option<&Index> {
        self.index().iter().find(|idx| idx.id == id)
    }
    pub fn push_index(&mut self, index: Index) {
        self.index.push(index)
    }
    pub fn pregap(&self) -> Option<&TimeStamp> {
        self.pregap.as_ref()
    }
    pub fn postgap(&self) -> Option<&TimeStamp> {
        self.postgap.as_ref()
    }
    pub fn title(&self) -> &Vec<String> {
        self.title.as_ref()
    }
    pub fn push_title(&mut self, title: String) {
        self.title.push(title)
    }
    pub fn performer(&self) -> &Vec<String> {
        &self.performer
    }
    pub fn push_performer(&mut self, performer: String) {
        self.performer.push(performer)
    }
    pub fn songwriter(&self) -> &Vec<String> {
        &self.songwriter
    }
    pub fn push_songwriter(&mut self, songwriter: String) {
        self.songwriter.push(songwriter)
    }
    pub fn set_pregep(&mut self, pregap: TimeStamp) -> Option<TimeStamp> {
        self.pregap.replace(pregap)
    }
    pub fn set_postgep(&mut self, postgap: TimeStamp) -> Option<TimeStamp> {
        self.postgap.replace(postgap)
    }
    pub fn isrc(&self) -> Option<&str> {
        self.isrc.as_deref()
    }
    pub fn set_isrc(&mut self, isrc: String) -> Option<String> {
        self.isrc.replace(isrc)
    }
    pub fn flags(&self) -> &Vec<String> {
        &self.flags
    }
    pub fn push_flag(&mut self, flag: String) {
        self.flags.push(flag)
    }
    pub fn push_flags<F, S>(&mut self, flags: F)
    where
        F: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.flags.extend(flags.into_iter().map(Into::into))
    }
}
impl FromStr for Track {
    type Err = InvalidArgument;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (tp, id) = delimited(utils::keyword("TRACK"), utils::number(2), tag(" "))(s)
            .map_err(|_| InvalidArgument::InvalidId)?;
        Ok(Self::new_unchecked(id, tp.to_owned()))
    }
}
impl ops::Index<usize> for Track {
    type Output = Index;

    fn index(&self, index: usize) -> &Self::Output {
        &self.index[index]
    }
}
impl TrackInfo {
    /// Constructs a new TrackInfo
    pub const fn new(name: String, format: String) -> Self {
        Self::with_tracks(name, format, Vec::new())
    }
    pub const fn with_tracks(name: String, format: String, tracks: Vec<Track>) -> Self {
        Self {
            name,
            format,
            tracks,
        }
    }
    pub fn tracks(&self) -> &Vec<Track> {
        &self.tracks
    }
    /// Searches for track by id
    ///
    /// `.tracks().get(...)` may be a better choice because it takes O(n) to find the track
    pub fn get_track(&self, id: u8) -> Option<&Track> {
        self.tracks().iter().find(|track| track.id() == id)
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
    pub fn push_track(&mut self, track: Track) {
        self.tracks.push(track)
    }
}
impl IntoIterator for TrackInfo {
    type Item = Track;

    type IntoIter = <Vec<Track> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.tracks.into_iter()
    }
}
impl<'a> IntoIterator for &'a TrackInfo {
    type Item = &'a Track;

    type IntoIter = std::slice::Iter<'a, Track>;

    fn into_iter(self) -> Self::IntoIter {
        self.tracks.iter()
    }
}
impl ops::Index<usize> for TrackInfo {
    type Output = Track;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tracks[index]
    }
}
