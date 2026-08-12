pub(in crate::sde) use error::ReadParseFailReason;
pub(in crate::sde) use lines_parse::{extract_from_lines_one, extract_from_lines_two};
#[cfg(feature = "sde-fs")]
pub(in crate::sde) use lines_search::first_in_lines;

mod error;
mod lines_parse;
mod lines_search;
