#![warn(missing_docs)]

//! # REEFAST
//! REEFAST is an engine built to simulate EVE Online ship and structure fits.
//!
//! It exposes various endpoints to fetch aggregated stats and conduct fit optimizations. Initial setup consists of
//! following steps:
//!
//! - you feed EVE data using an [`dh::DataHandler`](dh::DataHandler) implementation
//! - the engine converts the data into optimized internal format
//! - you compose fit objects and fetch data from there

pub mod consts;
pub mod defines;
pub mod dh;
pub mod dh_impls;
pub mod eve_type;
pub mod prelude;
