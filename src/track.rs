use anyhow::Error;
use anyhow::Result;
use nom::sequence::tuple;
use nom::sequence::preceded;
use nom::bytes::complete::tag_no_case as tag;
use nom::combinator::rest;
use std::str::FromStr;
use crate::time::Duration;
use crate::utils;

#[derive(Debug, Clone)]
pub struct Index {
    pub id: u8, // index id must between 1 and 99, this filed should be private
    pub begin_time: Duration,
}
#[derive(Debug, Clone, Default)]
pub struct Track {
    pub id: u8, // track-id must between 1 and 99
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
    pub fn new(id: u8, begin_time: Duration) -> Result<Self> {
        if id <= 99 {
            Ok(Self { id, begin_time })
        } else {
            Err(anyhow::format_err!("index-id must be between 1 and 99"))
        }
    }
}
impl FromStr for Index {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, (id, duration)) = tuple((preceded(tag("INDEX "), utils::take_digit2), preceded(tag(" "), rest)))(s)
            .map_err(|_| anyhow::anyhow!("error"))?;
        Ok(Self { id: id.parse()?, begin_time: duration.parse()? })
    }
}
impl Track {
    pub fn new(id: u8, format: String) -> Result<Self> {
        if id <= 99 {
            Ok(Self { id, format, ..Self::default() })
        } else {
            Err(anyhow::format_err!("track-id must be between 1 and 99"))
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
impl TrackInfo {
    pub const fn new(name: String, format: String) -> Self {
        Self::with_tracks(name, format, Vec::new())
    }
    pub const fn with_tracks(name: String, format: String, tracks: Vec<Track>) -> Self {
        Self { name, format, tracks }
    }
    pub fn last_track(&self) -> Option<&Track> {
        self.tracks.last()
    }
    pub fn last_track_mut(&mut self) -> Option<&mut Track> {
        self.tracks.last_mut()
    }
    pub fn push_track(&mut self , track: Track) {
        self.tracks.push(track)
    }
}