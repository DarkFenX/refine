#![feature(default_field_values)]
#![feature(if_let_guard)]

//! # refine
//! Refine is a library built to simulate EVE Online ship and citadel fits.
//!
//! It exposes various endpoints to fetch aggregated stats and conduct fit optimizations. Initial
//! setup consists of the following steps:
//!
//! - you feed an [`ed::EveDataHandler`](ed::EveDataHandler) implementation and an optional
//!   [`ad::AdaptedDataCacher`](ad::AdaptedDataCacher) implementation to the [`Src`](Src)
//!   constructor
//! - during [`Src`](Src) initialization, the library attempts to use cached adapted data. If its
//!   fingerprint (data version \+ library version at the time of cache generation) and current
//!   fingerprint (version of currently provided data + current library version) matched, cached
//!   data is attempted to be loaded. If cache couldn't be loaded or fingerprints mismatch, EVE data
//!   is fetched and converted into adapted data (this process is relatively heavy on both IO and
//!   CPU), which is then fed to adapted data cacher implementation.
//! - you create [`SolarSystem`](SolarSystem), and manipulate it to create fits with ships and
//!   items, and fetch data and stats

pub use api::{
    Ability, AbilityIter, AbilityMut, AddMode, AdjustableCount, AttrId, Autocharge, AutochargeMut, Booster, BoosterMut,
    Character, CharacterMut, Charge, ChargeMut, Coordinates, Direction, Drone, DroneMut, EffectId, EffectiveMutation,
    EffectiveMutationMut, Fighter, FighterMut, Fit, FitMut, FitSecStatus, Fleet, FleetMut, FullMAttr, FullMAttrIter,
    FullMAttrMut, FullSideEffect, FullSideEffectMut, FwEffect, FwEffectMut, Implant, ImplantMut, IncompleteMutation,
    IncompleteMutationMut, Item, ItemCommon, ItemMut, ItemMutCommon, MinionState, Module, ModuleIter, ModuleMut,
    ModuleState, Movement, MutIter, Mutation, MutationMut, Op, Proj, ProjEffect, ProjEffectMut, ProjIter, ProjMut,
    RangedProj, RangedProjIter, RangedProjMut, RawMAttr, RawMAttrIter, RawMAttrMut, Rig, RigMut, RmMode, Service,
    ServiceMut, ServiceState, Ship, ShipMut, SideEffect, SideEffectIter, SideEffectMut, SideEffectPartialStr,
    SideEffectStr, Skill, SkillMut, Stance, StanceMut, StubSideEffect, StubSideEffectMut, Subsystem, SubsystemMut,
    SwEffect, SwEffectMut,
};
pub use def::{
    AbilId, AttrVal, Count, CustomAttrId, CustomEffectId, DogmaEffectId, EveAttrId, FitId, FleetId, Idx, ItemGrpId,
    ItemId, ItemTypeId, SlotIndex, VERSION,
};
pub use lender::Lender;
pub use misc::{
    BreacherInfo, DpsProfile, Ecm, EffectInfo, EffectMode, FighterCountOverride, MiningAmount, ModRack, NpcProp,
    ProjRange, SecZone, SecZoneCorruption, SkillLevel, Spool,
};
pub use rd::Src;
pub use sol::SolarSystem;
pub use svc::calc::{Affector, CalcAttrVal, Modification};
pub use util::UnitInterval;

mod ac;
pub mod ad;
mod api;
mod dbg;
mod def;
mod ec;
pub mod ed;
pub mod err;
mod misc;
mod nd;
mod rd;
mod sol;
pub mod stats;
mod svc;
mod ud;
pub mod util;
pub mod val;
