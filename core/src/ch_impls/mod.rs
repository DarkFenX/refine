//! Cache handler implementations.

pub use json_file::JsonFileCHandler;
pub use ram_only::RamOnlyCHandler;

pub(self) mod common;
mod json_file;
mod ram_only;
