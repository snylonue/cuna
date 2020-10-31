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

/// Returns a str without UTF-8 bom
/// 
/// ```rust
/// let s = "﻿Hana is so cute"; // A str with BOM
/// assert_ne!(s, "Hana is so cute");
/// assert_eq!(cuna::trim_utf8_header(s), "Hana is so cute");
///```
#[inline]
pub fn trim_utf8_header(s: &str) -> &str {
    s.trim_start_matches('﻿')
}