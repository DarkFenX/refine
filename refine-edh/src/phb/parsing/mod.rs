pub(in crate::phb) use aliases::Key;
pub(in crate::phb) use error::ReadParseFailReason;
#[cfg(feature = "phb-file")]
pub(in crate::phb) use parse_array::ArrayIter;
pub(in crate::phb) use parse_keymap::{handle_keymap_one, handle_keymap_two};
pub(in crate::phb) use traits::{KeyMergeOne, KeyMergeTwo};

mod aliases;
mod error;
#[cfg(feature = "phb-file")]
mod parse_array;
mod parse_keymap;
mod recovery;
mod traits;
