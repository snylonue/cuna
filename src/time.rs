use failure::Error;
use nom::bytes::complete::take_until;
use nom::bytes::complete::take_while_m_n;
use nom::bytes::complete::tag;
use nom::character::is_digit;
use nom::sequence::preceded;
use nom::error::ErrorKind;
use nom::Err as NomErr;
use std::str::FromStr;

#[derive(Debug)]
pub struct Duration {
    seconds: u32,
    frames: u8,
}

impl Duration {
    pub fn new(minutes: u32, seconds: u32, frames: u8) -> Self {
        Self::from_msf_opt(minutes, seconds, frames).expect("Invaild time")
    }
    pub fn from_msf_opt(minutes: u32, seconds: u32, frames: u8) -> Option<Self> {
        if seconds >= 60 || frames >= 75 {
            None
        } else {
            Some(Self { seconds: minutes * 60 + seconds, frames })
        }
    }
    pub fn from_msf_force(mut minutes: u32, mut seconds: u32, mut frames: u8) -> Self {
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
        self.seconds
    }
    pub fn frames(&self) -> u32 {
        self.frames as u32
    }
}

impl FromStr for Duration {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let is_digit_char = |c| is_digit(c as u8);
        let err_msg = |_: NomErr<(_, ErrorKind)>| failure::err_msg("Invaild time");
        let (rest, minutes) = take_until(":")(s).map_err(err_msg)?;
        let (rest, seconds) = preceded(tag(":"), take_while_m_n(2, 2, is_digit_char))(rest).map_err(err_msg)?;
        let (_, frames) = preceded(tag(":"), take_while_m_n(2, 2, is_digit_char))(rest).map_err(err_msg)?;
        Ok(Self::from_msf_force(minutes.parse()?, seconds.parse()?, frames.parse()?))
    }
}
