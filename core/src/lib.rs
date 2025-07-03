#![feature(exact_size_is_empty)]
#![feature(default_field_values)]

//! # Reefast
//! Reefast is a library built to simulate EVE Online ship and structure fits.
//!
//! It exposes various endpoints to fetch aggregated stats and conduct fit optimizations. Initial
//! setup consists of the following steps:
//!
//! - you feed an [`ed::EveDataHandler`](ed::EveDataHandler) implementation and an
//!   [`ad::AdaptedDataHandler`](ad::AdaptedDataHandler) implementation to the [`Src`](Src)
//!   constructor
//! - during [`Src`](Src) initialization, the library attempts to load cached adapted data. If the
//!   cached data is loaded successfully, the library compares its fingerprint (data version \+
//!   library version at the time of cache generation) and current fingerprint (version of currently
//!   provided data + current library version). If cache couldn't be loaded or fingerprints
//!   mismatch, EVE data is fetched and converted into adapted data (this process is relatively
//!   heavy on both IO and CPU), which is then fed to adapted data handler implementation.
//! - you create [`SolarSystem`](SolarSystem), and manipulate it to create fits with ships and
//!   items, and fetch data and stats

pub use def::{
    AttrId, AttrVal, Count, CustomEffectId, DogmaEffectId, FitId, FleetId, Idx, ItemGrpId, ItemId, ItemTypeId,
    SlotIndex, VERSION,
};
pub use lender::Lender;
pub use misc::{
    AddMode, BreacherInfo, BreacherInfoError, DpsProfile, EffectId, EffectInfo, EffectMode, FighterCountOverride,
    FitSecStatus, MinionState, ModRack, ModuleState, OpInfo, ProjRange, RmMode, SecZone, SecZoneCorruption,
    ServiceState, SkillLevel, UnitInterval,
};
pub use sol::{
    SolarSystem,
    api::{
        Autocharge, AutochargeMut, Booster, BoosterMut, Character, CharacterMut, Charge, ChargeMut, Drone, DroneMut,
        EffectiveMutation, EffectiveMutationMut, Fighter, FighterMut, Fit, FitMut, Fleet, FleetMut, FullMAttr,
        FullMAttrIter, FullMAttrMut, FullSideEffect, FullSideEffectMut, FwEffect, FwEffectMut, Implant, ImplantMut,
        IncompleteMutation, IncompleteMutationMut, Item, ItemCommon, ItemMut, ItemMutCommon, Module, ModuleIter,
        ModuleMut, MutIter, Mutation, MutationMut, Proj, ProjEffect, ProjEffectMut, ProjIter, ProjMut, RangedProj,
        RangedProjIter, RangedProjMut, RawMAttr, RawMAttrIter, RawMAttrMut, Rig, RigMut, Service, ServiceMut, Ship,
        ShipMut, SideEffect, SideEffectIter, SideEffectMut, SideEffectPartialStr, SideEffectStr, Skill, SkillMut,
        Stance, StanceMut, StubSideEffect, StubSideEffectMut, Subsystem, SubsystemMut, SwEffect, SwEffectMut,
    },
};
pub use src::Src;
pub use svc::calc::{AffectorInfo, CalcAttrVal, ModificationInfo};

mod ac;
pub mod ad;
mod adg;
mod dbg;
mod def;
mod ec;
pub mod ed;
pub mod err;
mod misc;
mod ntt;
mod sol;
mod src;
pub mod stats;
mod svc;
mod uad;
pub mod util;
pub mod val;
