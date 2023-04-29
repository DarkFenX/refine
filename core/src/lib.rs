#![feature(drain_filter)]
#![feature(hash_drain_filter)]

//! # REEFAST
//! REEFAST is an engine built to simulate EVE Online ship and structure fits.
//!
//! It exposes various endpoints to fetch aggregated stats and conduct fit optimizations. Initial
//! setup consists of the following steps:
//!
//! - you feed EVE data using an [`dh::DataHandler`](crate::dh::DataHandler) implementation
//! - the engine converts the data into optimized internal format
//! - you compose fit objects and fetch data from there

extern crate core;

pub use consts::{OrdAddMode, State};
pub use defines::{ReeFloat, ReeId, ReeIdx, ReeInt, REEINT_MAX, REEINT_MIN, VERSION};
pub use src::Src;
pub use ss::{IdData, SolarSystem};
pub use util::err_res::public::{Error, ErrorKind, Result};
pub(crate) use util::err_res::{
    internal::{IntError, IntResult},
    public::FromKind,
};

pub(crate) mod cg;
pub mod ch;
pub mod ch_impls;
pub mod consts;
pub mod ct;
mod defines;
pub mod dh;
pub mod dh_impls;
pub mod prelude;
mod src;
mod ss;
mod util;
