#![feature(drain_filter)]

//! # REEFAST
//! REEFAST is an engine built to simulate EVE Online ship and structure fits.
//!
//! It exposes various endpoints to fetch aggregated stats and conduct fit optimizations. Initial
//! setup consists of the following steps:
//!
//! - you feed EVE data using an [`dh::DataHandler`](crate::dh::DataHandler) implementation
//! - the engine converts the data into optimized internal format
//! - you compose fit objects and fetch data from there

pub use defines::{ReeFloat, ReeInt, REEINT_MAX, REEINT_MIN, VERSION};
pub use fit::Fit;
pub(crate) use item::FitChild;
pub use item::{ItemBase, Ship};
pub use src::SrcMgr;
pub use ss::SolarSystem;
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
mod fit;
mod item;
pub mod prelude;
mod src;
mod ss;
mod util;
