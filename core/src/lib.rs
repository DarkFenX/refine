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

pub use consts::{ModRack, OrdAddMode, State};
pub use defs::{ReeFloat, ReeId, ReeIdx, ReeInt, REEINT_MAX, REEINT_MIN, VERSION};
pub use src::Src;
pub use ss::{
    AttrVal, BoosterInfo, CharacterInfo, ChargeInfo, DroneInfo, FighterInfo, ImplantInfo, ItemInfo, ModuleInfo,
    RigInfo, ShipInfo, SkillInfo, SolarSystem, StanceInfo, SubsystemInfo, SwEffectInfo,
};
pub use util::{Error, ErrorKind, Result};

pub(crate) mod cg;
pub mod ch;
pub mod ch_impls;
pub mod consts;
pub mod ct;
mod defs;
pub mod dh;
pub mod dh_impls;
pub mod prelude;
mod src;
mod ss;
mod util;
