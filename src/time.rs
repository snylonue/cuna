use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::combinator::map;
use std::str::FromStr;
use std::fmt;
use crate::utils::number;
use crate::error::ParseError;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Duration {
    seconds: u32,
    frames: u8,
}

impl Duration {
    /// Constructs a new Duration with minutes, seconds and frames
    /// 
    /// # Panics
    ///
    /// Panics if seconds >= 60 or frames >= 75
    pub fn new(minutes: u32, seconds: u32, frames: u32) -> Self {
        Self::from_msf_opt(minutes, seconds, frames).expect("Invaild time")
    }
    /// Constructs a new Duration with minutes, seconds and frames, or returns None if seconds >= 60 or frames >= 75
    pub fn from_msf_opt(minutes: u32, seconds: u32, frames: u32) -> Option<Self> {
        if seconds >= 60 || frames >= 75 {
            None
        } else {
            Some(Self { seconds: minutes * 60 + seconds, frames: frames as u8 })
        }
    }
    /// Constructs a new Duration with minutes, seconds and frames
    /// 
    /// It never panics, if seconds or frames are too big, they will be carried over into a larger unit
    pub fn from_msf(mut minutes: u32, mut seconds: u32, mut frames: u32) -> Self {
        seconds += (frames / 75) as u32;
        frames %= 75;
        minutes += seconds / 60;
        seconds %= 60;
        Self::new(minutes, seconds, frames)
    }
    pub fn minutes(&self) -> u32 {
        self.seconds / 60 as u32
    }
    pub fn seconds(&self) -> u32 {
        self.seconds % 60
    }
    pub fn frames(&self) -> u32 {
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
}
impl FromStr for Duration {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err_msg = |_| ParseError::syntax_error(s, "invaild duration");
        let (_, (minutes, seconds, frames)) = tuple((
            terminated(map(digit1, |d: &str| d.parse().unwrap()), tag(":")),
            terminated(number(2), tag(":")), 
            number(2)
        ))(s).map_err(err_msg)?;
        // minutes, seconds and frames are confirmed to be vaild u8 value
        Ok(Self::new(minutes, seconds as u32, frames as u32))
    }
}
impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}:{:0>2}", self.minutes(), self.seconds(), self.frames())
    }
}