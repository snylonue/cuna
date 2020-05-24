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

impl Header {
    pub fn push_title(&mut self, title: String) {
        self.title.get_or_insert_with(|| Vec::with_capacity(1)).push(title)
    }
    pub fn push_performer(&mut self, performer: String) {
        self.performer.get_or_insert_with(|| Vec::with_capacity(1)).push(performer)
    }
    pub fn push_songwriter(&mut self, songwriter: String) {
        self.songwriter.get_or_insert_with(|| Vec::with_capacity(1)).push(songwriter)
    }
    pub fn set_catalog(&mut self, catalog: u64) -> Result<Option<u64>>{
        if len(catalog) == 13 {
            Ok(self.catalog.replace(catalog))
        } else {
            Err(anyhow::format_err!("Invaild catalog"))
        }
    }
    pub fn set_cdtextfile(&mut self, cdtextfile: String) -> Option<String> {
        self.cdtextfile.replace(cdtextfile)
    }
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
fn len(d: u64) -> usize {
    d.to_string().len()
}
