use anyhow::Result;

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

fn len(d: u64) -> usize {
    d.to_string().len()
}
