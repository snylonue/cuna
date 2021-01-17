use crate::error::ParseError;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Header {
    pub title: Vec<String>,
    pub performer: Vec<String>,
    pub songwriter: Vec<String>,
    pub(crate) catalog: Option<u64>,
    pub cdtextfile: Option<String>,
}

impl Header {
    pub fn title(&self) -> &Vec<String> {
        &self.title
    }
    pub fn title_mut(&mut self) -> &mut Vec<String> {
        &mut self.title
    }
    pub fn push_title(&mut self, title: String) {
        self.title.push(title)
    }
    pub fn performer(&self) -> &Vec<String> {
        &self.performer
    }
    pub fn performer_mut(&mut self) -> &mut Vec<String> {
        &mut self.performer
    }
    pub fn push_performer(&mut self, performer: String) {
        self.performer.push(performer)
    }
    pub fn songwriter(&self) -> &Vec<String> {
        &self.songwriter
    }
    pub fn songwriter_mut(&mut self) -> &mut Vec<String> {
        &mut self.songwriter
    }
    pub fn push_songwriter(&mut self, songwriter: String) {
        self.songwriter.push(songwriter)
    }
    pub fn catalog(&self) -> Option<u64> {
        self.catalog
    }
    pub fn set_catalog(&mut self, catalog: u64) -> Result<Option<u64>, ParseError> {
        if len(catalog) == 13 {
            Ok(self.catalog.replace(catalog))
        } else {
            Err(ParseError::syntax_error(catalog, "invalid catalog"))
        }
    }
    pub fn cdtextfile(&self) -> Option<&str> {
        self.cdtextfile.as_deref()
    }
    pub fn set_cdtextfile(&mut self, cdtextfile: String) -> Option<String> {
        self.cdtextfile.replace(cdtextfile)
    }
}

#[inline]
fn len(d: u64) -> usize {
    (d as f32).log10() as usize + 1
}
