pub(in crate::sde) use error::ReadParseFailReason;
#[cfg(feature = "sde-fs")]
pub(in crate::sde) use parse_array::find_in_array;
pub(in crate::sde) use parse_lines::{extract_from_lines_one, extract_from_lines_two};

mod error;
#[cfg(feature = "sde-fs")]
mod parse_array;
mod parse_lines;
