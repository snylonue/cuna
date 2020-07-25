use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::combinator::map;
use std::str::FromStr;
use std::fmt;
use std::time::Duration;
use crate::utils::number;
use crate::error::ParseError;

#[derive(Debug, PartialEq, Eq, Clone, Default, Hash, Copy)]
pub struct TimeStamp {
    seconds: u32,
    frames: u8,
}

impl TimeStamp {
    /// Constructs a new TimeStamp with minutes, seconds and frames
    /// 
    /// # Panics
    ///
    /// Panics if seconds >= 60 or frames >= 75
    pub fn new(minutes: u32, seconds: u32, frames: u32) -> Self {
        Self::from_msf_opt(minutes, seconds, frames).expect("Invalid time")
    }
    /// Constructs a new TimeStamp with minutes, seconds and frames, or returns None if seconds >= 60 or frames >= 75
    pub fn from_msf_opt(minutes: u32, seconds: u32, frames: u32) -> Option<Self> {
        if seconds >= 60 || frames >= 75 {
            None
        } else {
            Some(Self { seconds: minutes * 60 + seconds, frames: frames as u8 })
        }
    }
    /// Constructs a new TimeStamp with minutes, seconds and frames
    /// 
    /// It never panics, if seconds or frames are too big, they will be carried over into a larger unit
    pub const fn from_msf(minutes: u32, seconds: u32, frames: u32) -> Self {
        Self { seconds: minutes * 60 + seconds + (frames / 75), frames: (frames % 75) as u8 }
    }
    pub const fn minutes(&self) -> u32 {
        self.seconds / 60 as u32
    }
    pub const fn seconds(&self) -> u32 {
        self.seconds % 60
    }
    pub const fn frames(&self) -> u32 {
        self.frames as u32
    }
    pub fn set_minutes(&mut self, minutes: u32) {
        self.seconds = self.seconds() + minutes * 60;
    }
    /// # Panics
    ///
    /// Panics if seconds >= 60
    pub fn set_seconds(&mut self, seconds: u32) {
        assert!(seconds < 60);
        self.seconds = self.minutes() * 60 + seconds;
    }
    /// # Panics
    ///
    /// Panics if frames >= 75
    pub fn set_frames(&mut self, frames: u32) {
        assert!(frames < 75);
        self.frames = frames as u8;
    }
    pub const fn as_seconds(&self) -> u32 {
        self.seconds
    }
    pub const fn as_frames(&self) -> u32 {
        self.as_seconds() * 75 + self.frames()
    }
}
impl FromStr for TimeStamp {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, (minutes, seconds, frames)) = tuple((
            terminated(map(digit1, |d: &str| d.parse().unwrap()), tag(":")),
            terminated(number(2), tag(":")), 
            number(2)
        ))(s).map_err(|_| ParseError::syntax_error(s, "invalid timestamp"))?;
        Ok(Self::new(minutes, seconds, frames))
    }
}
impl fmt::Display for TimeStamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}:{:0>2}", self.minutes(), self.seconds(), self.frames())
    }
}
impl From<TimeStamp> for Duration {
    fn from(ti: TimeStamp) -> Duration {
        Duration::from_secs(ti.seconds() as u64) + Duration::from_millis((ti.frames() * 40 / 3) as u64)
    }
}
impl From<Duration> for TimeStamp {
    fn from(dr: Duration) -> Self {
        let secs = dr.as_secs();
        let frames = dr.subsec_millis() * 3 / 40;
        Self::from_msf(0, secs as u32, frames)
    }
}