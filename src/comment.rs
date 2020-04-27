use std::iter::FromIterator;
use crate::utils;

#[derive(Debug)]
pub struct Comment(pub Vec<String>);

impl Comment {
    pub fn new(s: &str) -> Self {
        s.lines()
            .filter_map(|s| utils::parse_line(s, "REM")
                .ok()
                .map(|(c, _)| c.trim())
            )
            .collect()
    }
}
impl<S: Into<String>> FromIterator<S> for Comment {
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        Self(iter.into_iter().map(Into::into).collect())
    }
}
