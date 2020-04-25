use failure::Error;
use nom::Err as NomErr;
use std::str::FromStr;
use std::collections::BTreeMap;
use crate::utils;
use crate::HanaResult;

#[derive(Debug, Default)]
pub struct Header {
    pub title: Option<Vec<String>>,
    pub performer: Option<Vec<String>>,
    pub songwriter: Option<Vec<String>>,
    pub catalog: Option<u64>,
    pub cdtextfile: Option<String>,
}
impl FromStr for Header {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_header(s)
    }
}

fn parse_header_from_iter<'a, I>(iter: I) -> HanaResult<Header>
    where I: IntoIterator<Item = &'a str>
{
    let mut headers = BTreeMap::new();
    for line in iter.into_iter() {
        match tags!("title", "performer", "songwriter", "catalog", "cdtextfile")(line) {
            Ok((content, command)) => match utils::quotec(content.trim()) {
                Ok((_, content)) | Err(NomErr::Error((content, _))) => headers.entry(command.to_ascii_lowercase())
                    .or_insert_with(|| Vec::with_capacity(1))
                    .push(content.to_owned()),
                _ => return Err(failure::err_msg("Unexcept error while parsing header")),
            },
            _ => {},
        }
    }
    let [title, performer, songwriter, catalog, cdtextfile] = get!(headers,
        (title, performer, songwriter, catalog, cdtextfile));
    let header = Header {
        title,
        performer,
        songwriter,
        catalog: match catalog {
            Some(s) if s.len() == 1 && s[0].len() == 13 => Some(s[0].parse()?),
            Some(_) => return Err(failure::err_msg("Invaild catalog")),
            None => None,
        },
        cdtextfile: cdtextfile.map(|mut v| v.pop()).flatten(),
    };
    Ok(header)
}
fn parse_header(s: &str) -> HanaResult<Header> {
    parse_header_from_iter(s.lines())
}
