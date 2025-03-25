#![feature(exact_size_is_empty)]
#![feature(variant_count)]

//! # Reefast
//! Reefast is a library built to simulate EVE Online ship and structure fits.
//!
//! It exposes various endpoints to fetch aggregated stats and conduct fit optimizations. Initial
//! setup consists of the following steps:
//!
//! - you feed an [`ed::EveDataHandler`](crate::ed::EveDataHandler) implementation and an
//!   [`ad::AdaptedDataHandler`](crate::ad::AdaptedDataHandler) implementation to the
//!   [`Src`](crate::Src) constructor
//! - during [`Src`](crate::Src) initialization, the library attempts to load cached adapted data.
//!   If the cached data is loaded successfully, the library compares its fingerprint (data version
//!   \+ library version at the time of cache generation) and current fingerprint (version of
//!   currently provided data + current library version). If cache couldn't be loaded or
//!   fingerprints mismatch, EVE data is fetched and converted into adapted data (this process is
//!   relatively heavy on both IO and CPU), which is then fed to adapted data handler
//!   implementation.
//! - you create [`SolarSystem`](crate::SolarSystem), and manipulate it to create fits with ships
//!   and items, and fetch data and stats

pub use defs::VERSION;
pub use sol::{
    AddMode, AttrId, AttrVal, Count, DmgProfile, EffectId, EffectInfo, EffectMode, FitId, FleetId, Idx, ItemGrpId,
    ItemId, ItemTypeId, ModRack, MutaRoll, OpInfo, RmMode, SecZone, SecZoneCorruption, SkillLevel, SlotIndex,
    SolarSystem,
    info::{
        AttrMutationInfo, AutochargeInfo, BoosterInfo, CharacterInfo, ChargeInfo, DroneInfo, FighterInfo, FitInfo,
        FleetInfo, FwEffectInfo, ImplantInfo, ItemInfo, ItemMutationInfo, ModuleInfo, ProjEffectInfo, ProjInfo,
        RigInfo, ServiceInfo, ShipInfo, SideEffectInfo, SideEffectStr, SkillInfo, StanceInfo, SubsystemInfo,
        SwEffectInfo,
    },
    svc::calc::{AffectorInfo, CalcAttrVal, ModificationInfo},
    uad::{
        ItemAddAttrMutation, ItemAddMutation, ItemAttrMutationValue, ItemChangeAttrMutation, MinionState, ModuleState,
        ServiceState,
    },
};
pub use src::Src;

mod ac;
pub mod ad;
mod adg;
mod config;
mod defs;
mod ec;
pub mod ed;
pub mod err;
mod sol;
mod src;
pub mod util;
pub mod val;
