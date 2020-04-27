use crate::time::Duration;

#[derive(Debug)]
pub struct Index {
    pub id: u8, // index id must between 1 and 99
    pub begin_time: Duration,
}
#[derive(Debug)]
pub struct Track {
    pub id: u8, // truck-id must between 1 and 99
    pub track_type: String,
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
pub struct FileTrucks {
    pub name: String,
    pub data_type: String,
    pub trucks: Vec<Track>,
}