#![feature(drain_filter)]
#![feature(hash_drain_filter)]

//! # Reefast
//! Reefast is a library built to simulate EVE Online ship and structure fits.
//!
//! It exposes various endpoints to fetch aggregated stats and conduct fit optimizations. Initial
//! setup consists of the following steps:
//!
//! - you feed EVE data using an [`ed::EveDataHandler`](crate::ed::EveDataHandler) implementation
//! - the engine adapts the data into format which is optimized for use by the library
//! - library feeds it to an [`ad::AdaptedDataHandler`](crate::ad::AdaptedDataHandler)
//! implementation and later uses it to fetch adapted data as needed
//! - you compose solar system object, and manipulate it to create fits with ships and items, and
//! fetch data and stats
//!
//! The data adaptation step can be skipped, if an
//! [`ad::AdaptedDataHandler`](crate::ad::AdaptedDataHandler) you use caches data, and the library
//! considers that cached data is still valid.

pub use consts::{ModRack, OrdAddMode, State};
pub use defs::{ReeFloat, ReeId, ReeIdx, ReeInt, REEINT_MAX, REEINT_MIN, VERSION};
pub use src::Src;
pub use ss::{AttrVal, SolarSystem};
pub use ssn::{
    BoosterInfo, CharacterInfo, ChargeInfo, DroneInfo, FighterInfo, ImplantInfo, ItemInfo, ModuleInfo, RigInfo,
    ShipInfo, SkillInfo, StanceInfo, SubsystemInfo, SwEffectInfo,
};
pub use util::{Error, ErrorKind, Result};

pub mod ad;
mod adg;
pub mod consts;
mod defs;
pub mod ed;
pub mod prelude;
mod src;
mod ss;
mod ssi;
pub mod ssn;
mod util;
