use std::iter::FromIterator;
use crate::utils;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Comment(pub Vec<String>);

impl Comment {
    pub fn new(s: &str) -> Self {
        s.lines()
            .filter_map(|s| {
                utils::keyword("REM")(s)
                .ok()
                .map(|(c, _)| c)
            })
            .collect()
    }
    pub fn push(&mut self, s: String) {
        self.0.push(s)
    }
}
impl<S: Into<String>> FromIterator<S> for Comment {
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        Self(iter.into_iter().map(Into::into).collect())
    }
}
impl IntoIterator for Comment {
    type Item = String;
    type IntoIter = <Vec<String> as IntoIterator>::IntoIter;
    
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}