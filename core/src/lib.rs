#![feature(drain_filter)]
#![feature(hash_drain_filter)]

//! # Reefast
//! Reefast is a library built to simulate EVE Online ship and structure fits.
//!
//! It exposes various endpoints to fetch aggregated stats and conduct fit optimizations. Initial
//! setup consists of the following steps:
//!
//! - you feed an [`ed::EveDataHandler`](crate::ed::EveDataHandler) implementation and an
//! [`ad::AdaptedDataHandler`](crate::ad::AdaptedDataHandler) implementation to the
//! [`Src`](crate::Src) constructor
//! - during [`Src`](crate::Src) initialization, the library attempts to load cached adapted data.
//! If the cached data is loaded successfully, the library compares its fingerprint (data version +
//! library version at the time of cache generation) and current fingerprint (version of currently
//! provided data + current library version). If cache couldn't be loaded or fingerprints mismatch,
//! EVE data is fetched and converted into adapted data (this process is relatively heavy on both IO
//! and CPU), which is then fed to adapted data handler implementation.
//! - you create [`SolarSystem`](crate::SolarSystem), and manipulate it to create fits with ships
//! and items, and fetch data and stats

pub use consts::{EffectMode, ModRack, OrdAddMode, State};
pub use defs::{AttrId, EffectId, ReeFloat, ReeId, ReeIdx, ReeInt, REEINT_MAX, REEINT_MIN, VERSION};
pub use src::Src;
pub use ss::{
    info::{
        SsBoosterInfo, SsCharacterInfo, SsChargeInfo, SsDroneInfo, SsFighterInfo, SsImplantInfo, SsItemInfo,
        SsModuleInfo, SsRigInfo, SsShipInfo, SsSkillInfo, SsStanceInfo, SsSubsystemInfo, SsSwEffectInfo,
    },
    EffectInfo, SolarSystem, SsAttrVal,
};
pub use util::{Error, ErrorKind, Result};

pub mod ad;
mod adg;
pub mod consts;
mod defs;
pub mod ed;
mod src;
mod ss;
mod util;
