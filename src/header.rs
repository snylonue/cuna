use anyhow::Error;
use anyhow::Result;
use std::str::FromStr;
use std::collections::BTreeMap;
use crate::utils;

#[derive(Debug, Clone, Default)]
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

pub(crate) fn parse_header_lines<'a, I>(iter: I) -> Result<Header>
    where I: IntoIterator<Item = &'a str>
{
    let mut headers = BTreeMap::new();
    for line in iter {
        match tags!("title", "performer", "songwriter", "catalog", "cdtextfile")(line) {
            Ok((content, command)) => match utils::quote_opt(content) {
                Ok((_, content)) => headers.entry(command.to_ascii_lowercase())
                    .or_insert_with(|| Vec::with_capacity(1))
                    .push(content.to_owned()),
                _ => return Err(anyhow::anyhow!("Unexcept error while parsing header")),
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
            Some(_) => return Err(anyhow::anyhow!("Invaild or too many catalog(s)")),
            None => None,
        },
        cdtextfile: cdtextfile.map(|mut v| v.pop()).flatten(),
    };
    Ok(header)
}
fn parse_header(s: &str) -> Result<Header> {
    parse_header_lines(s.lines())
}
