#![feature(drain_filter)]
#![feature(hash_drain_filter)]

//! # Reefast
//! Reefast is an engine built to simulate EVE Online ship and structure fits.
//!
//! It exposes various endpoints to fetch aggregated stats and conduct fit optimizations. Initial
//! setup consists of the following steps:
//!
//! - you feed EVE data using an [`dh::DataHandler`](crate::edh::EveDataHandler) implementation
//! - the engine converts the data into optimized internal format
//! - you compose fit objects and fetch data from there

extern crate core;

pub use consts::{ModRack, OrdAddMode, State};
pub use defs::{ReeFloat, ReeId, ReeIdx, ReeInt, REEINT_MAX, REEINT_MIN, VERSION};
pub use src::Src;
pub use ss::{AttrVal, SolarSystem};
pub use ssn::{
    BoosterInfo, CharacterInfo, ChargeInfo, DroneInfo, FighterInfo, ImplantInfo, ItemInfo, ModuleInfo, RigInfo,
    ShipInfo, SkillInfo, StanceInfo, SubsystemInfo, SwEffectInfo,
};
pub use util::{Error, ErrorKind, Result};

pub(crate) mod adg;
pub mod adh;
pub mod adt;
pub mod consts;
mod defs;
pub mod edh;
pub mod edt;
pub mod prelude;
mod src;
mod ss;
mod ssi;
pub mod ssn;
mod util;
