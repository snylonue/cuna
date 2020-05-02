use failure::Error;
use nom::branch::alt;
use nom::sequence::tuple;
use nom::sequence::preceded;
use nom::bytes::complete::take_until;
use nom::bytes::complete::tag_no_case as tag;
use nom::combinator::rest;
use nom::Err as NomErr;
use std::collections::BTreeMap;
use std::str::FromStr;
use crate::HanaResult;
use crate::time::Duration;
use crate::utils;

#[derive(Debug, Clone)]
pub struct Index {
    pub id: u8, // index id must between 1 and 99
    pub begin_time: Duration,
}
#[derive(Debug, Clone)]
pub struct Track {
    pub id: u8, // truck-id must between 1 and 99
    pub track_type: String,
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
pub struct FileTracks {
    pub name: String,
    pub data_type: String,
    pub tracks: Vec<Track>,
}

impl FromStr for Index {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, (id, duration)) = tuple((preceded(tag("INDEX "), utils::take_digit2), preceded(tag(" "), rest)))(s)
            .map_err(|_| failure::err_msg("error"))?;
        Ok(Self { id: id.parse()?, begin_time: duration.parse()? })
    }
}

fn parse_track_lines<'a, I>(lines: I) -> HanaResult<Track>
    where I: Iterator<Item = &'a str>
{
    let mut commands = BTreeMap::new();
    let mut indexs = Vec::new();
    let mut lines = lines.into_iter();
    let first_line = lines.next().unwrap();
    let (_, (id, track_type)) = preceded(tag("TRACK "), tuple((utils::take_digit2, preceded(tag(" "), rest))))(first_line.trim()).unwrap();
    for line in lines {
        match tags!("title", "performer", "songwriter", "isrc", "flags", "pregap", "postgap")(line.trim()) {
            Ok((content, command)) => match utils::quote_opt(content.trim()) {
                Ok((_, content)) => commands.entry(command.to_ascii_lowercase())
                    .or_insert_with(|| Vec::with_capacity(1))
                    .push(content),
                _ => return Err(failure::err_msg("Unexcept error while parsing track")),
            },
            Err(NomErr::Error((content, _))) => indexs.push(content),
            _ => return Err(failure::err_msg("Unexcept error while parsing track")),
        }
    }
    let [title, performer, songwriter, isrc, flags, pregap, postgap] = get!(commands,
        (title, performer, songwriter, isrc, flags, pregap, postgap));
    let index = indexs.into_iter()
        .map(FromStr::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    let to_owned = |v: Vec<&str>| v.into_iter().map(|s| s.to_owned()).collect();
    let track = Track {
        id: id.parse()?,
        track_type: track_type.to_owned(),
        index: index,
        pregap: match pregap {
            Some(pregaps) if pregaps.len() == 1 => Some(pregaps[0].parse()?),
            Some(_) => return Err(failure::err_msg("Too many pregaps")),
            None => None,
        },
        postgap: match postgap {
            Some(postgaps) if postgaps.len() == 1 => Some(postgaps[0].parse()?),
            Some(_) => return Err(failure::err_msg("Too many postgap")),
            None => None,
        },
        title: title.map(to_owned),
        performer: performer.map(to_owned),
        songwriter: songwriter.map(to_owned),
        isrc: match isrc {
            Some(isrcs) if isrcs.len() == 1 => Some(isrcs[0].parse()?),
            Some(_) => return Err(failure::err_msg("Too many isrcs")),
            None => None,
        },
        flags: flags.map(to_owned),
    };
    Ok(track)
}
pub(crate) fn parse_filetracks_lines<'a, I>(lines: I) -> HanaResult<FileTracks>
    where I: Iterator<Item = &'a str> + Clone
{
    let mut lines = lines.into_iter();
    let first_line = lines.next().unwrap();
    let (name, data_type) = preceded(tag("FILE "), alt((utils::quote_opt, take_until(" "))))(first_line)
        .map(|(dt, n)| (n, dt.trim_start()))
        .unwrap();
    let (_, tracks) = utils::scope(lines).unwrap();
    let tracks = tracks.into_iter()
        .map(IntoIterator::into_iter)
        .map(parse_track_lines)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(FileTracks { name: name.to_owned(), data_type: data_type.to_owned(), tracks })
}
pub fn parse_filetracks<S: AsRef<str>>(_s: S) -> HanaResult<FileTracks> {
    unimplemented!()
}