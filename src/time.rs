use failure::Error;
use nom::bytes::complete::take_while_m_n;
use nom::bytes::complete::tag;
use nom::character::is_digit;
use nom::character::complete::digit1;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::error::ErrorKind;
use nom::Err as NomErr;
use std::str::FromStr;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Duration {
    seconds: u32,
    frames: u8,
}

impl Duration {
    /// # Panics
    ///
    /// Panics if seconds >= 60 or frames >= 75
    pub fn new(minutes: u32, seconds: u32, frames: u32) -> Self {
        Self::from_msf_opt(minutes, seconds, frames).expect("Invaild time")
    }
    pub fn from_msf_opt(minutes: u32, seconds: u32, frames: u32) -> Option<Self> {
        if seconds >= 60 || frames >= 75 {
            None
        } else {
            Some(Self { seconds: minutes * 60 + seconds, frames: frames as u8 })
        }
    }
    pub fn from_msf_force(mut minutes: u32, mut seconds: u32, mut frames: u32) -> Self {
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
}

impl FromStr for Duration {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let take_digit2 = |s| take_while_m_n(2, 2, |c| is_digit(c as u8))(s);
        let err_msg = |_: NomErr<(_, ErrorKind)>| failure::err_msg("Invaild time");
        let (_, (minutes, seconds, frames)) = tuple((digit1, preceded(tag(":"), take_digit2), preceded(tag(":"), take_digit2)))(s).map_err(err_msg)?;
        Ok(Self::from_msf_force(minutes.parse()?, seconds.parse()?, frames.parse()?))
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:0>2}:{:0>2}:{:0>2}", self.minutes(), self.seconds(), self.frames)
    }
}