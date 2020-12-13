pub mod comment;
pub mod cuna;
pub mod error;
pub mod header;
pub mod parser;
pub mod time;
pub mod track;
pub mod utils;

pub use crate::cuna::Cuna;
pub use crate::cuna::Cuna as CueSheet;

/// Returns a str without UTF-8 bom
///
/// ```rust
/// let s = "﻿Hana is so cute"; // An str with BOM
/// assert_ne!(s, "Hana is so cute");
/// assert_eq!(cuna::trim_utf8_header(s), "Hana is so cute");
/// ```
#[inline]
pub fn trim_utf8_header(s: &str) -> &str {
    s.trim_start_matches('﻿')
}
