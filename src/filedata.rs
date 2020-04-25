use crate::time::Duration;

#[derive(Debug)]
pub struct Index {
    pub id: u8, // index id must between 1 and 99
    pub begin_time: Duration,
}
#[derive(Debug)]
pub struct Track {
    pub id: u8, // truck-id must between 1 and 99
    pub ttype: String,
    pub index: Vec<Index>,
    pub pregap: Option<String>,
    pub postgap: Option<String>,
    pub title: Option<String>,
    pub performer: Option<String>,
    pub songwriter: Option<String>,
    pub isrc: Option<String>,
    pub flags: Option<Vec<String>>
}
#[derive(Debug)]
pub struct FileInfo {
    pub name: String,
    pub ftype: String,
    pub trucks: Vec<Track>,
}