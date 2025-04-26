#![feature(exact_size_is_empty)]

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
pub use lender::Lender;
pub use sol::{
    AddMode, AttrId, AttrVal, BreacherInfo, Count, CustomEffectId, DogmaEffectId, DpsProfile, EffectId, EffectInfo,
    EffectMode, FitId, FleetId, Idx, ItemGrpId, ItemId, ItemTypeId, ModRack, MutaRoll, OpInfo, RmMode, SecStatus,
    SecZone, SecZoneCorruption, SkillLevel, SlotIndex, SolarSystem,
    api::{
        Autocharge, AutochargeMut, Booster, BoosterMut, Character, CharacterMut, Charge, ChargeMut, Drone, DroneMut,
        Fighter, FighterMut, Fit, FitMut, Fleet, FleetMut, FullSideEffect, FullSideEffectMut, FwEffect, FwEffectMut,
        Implant, ImplantMut, Item, ItemCommon, ItemMut, ItemMutCommon, Module, ModuleIter, ModuleMut, Proj, ProjEffect,
        ProjEffectMut, ProjMut, RangedProj, RangedProjMut, Rig, RigMut, Service, ServiceMut, Ship, ShipMut, SideEffect,
        SideEffectIter, SideEffectMut, SideEffectPartialStr, SideEffectStr, Skill, SkillMut, Stance, StanceMut,
        StubSideEffect, StubSideEffectMut, Subsystem, SubsystemMut, SwEffect, SwEffectMut,
    },
    info::{AttrMutationInfo, ItemMutationInfo, ProjInfo},
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
