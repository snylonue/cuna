use crate::error::ParseError;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Header {
    pub title: Option<Vec<String>>,
    pub performer: Option<Vec<String>>,
    pub songwriter: Option<Vec<String>>,
    pub(crate) catalog: Option<u64>,
    pub cdtextfile: Option<String>,
}

impl Header {
    pub fn title(&self) -> &Option<Vec<String>> {
        &self.title
    }
    pub fn push_title(&mut self, title: String) {
        self.title.get_or_insert_with(|| Vec::with_capacity(1)).push(title)
    }
    pub fn performer(&self) -> &Option<Vec<String>> {
        &self.performer
    }
    pub fn push_performer(&mut self, performer: String) {
        self.performer.get_or_insert_with(|| Vec::with_capacity(1)).push(performer)
    }
    pub fn songwriter(&self) -> &Option<Vec<String>> {
        &self.songwriter
    }
    pub fn push_songwriter(&mut self, songwriter: String) {
        self.songwriter.get_or_insert_with(|| Vec::with_capacity(1)).push(songwriter)
    }
    pub fn catalog(&self) -> &Option<u64> {
        &self.catalog
    }
    pub fn set_catalog(&mut self, catalog: u64) -> Result<Option<u64>, ParseError>{
        if len(catalog) == 13 {
            Ok(self.catalog.replace(catalog))
        } else {
            Err(ParseError::syntax_error(catalog, "invaild catalog"))
        }
    }
    pub fn cdtextfile(&self) -> &Option<String> {
        &self.cdtextfile
    }
    pub fn set_cdtextfile(&mut self, cdtextfile: String) -> Option<String> {
        self.cdtextfile.replace(cdtextfile)
    }
}

fn len(d: u64) -> usize {
    (d as f32).log10() as usize + 1
}
