pub mod utils;
pub mod error;
pub mod time;
pub mod header;
pub mod track;
pub mod comment;
pub mod parser;
pub mod cuna;

pub use crate::cuna::Cuna;
pub use crate::cuna::Cuna as CueSheet;

#[inline]
pub fn trim_utf8_header(s: &str) -> &str {
    s.trim_start_matches('﻿')
}