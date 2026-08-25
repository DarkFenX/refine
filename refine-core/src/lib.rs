#![feature(default_field_values)]
#![feature(error_reporter)]
#![feature(structural_match)]
#![feature(variant_count)]
#![feature(step_trait)]
#![feature(option_reduce)]
#![feature(integer_casts)]
#![feature(const_trait_impl)]
#![feature(const_cmp)]
#![feature(const_ops)]
#![feature(const_convert)]
#![feature(const_default)]
#![feature(const_result_trait_fn)]
#![cfg_attr(feature = "serde", feature(cfg_eval))]

//! # refine
//! Refine is a library built to simulate EVE Online ship and citadel fits.
//!
//! It exposes various endpoints to fetch aggregated stats and conduct fit optimizations. Initial
//! setup consists of the following steps:
//!
//! - you feed an [`ed::EveDataHandler`] and an optional [`ad::AdaptedDataCacher`] to the [`Src`]
//!   constructor. Both are wrappers which carry your [`ed::EveDataHandlerCore`] and
//!   [`ad::AdaptedDataCacherCore`] implementations;
//! - during [`Src`] initialization, the library attempts to use cached adapted data. If its
//!   fingerprint (data version \+ library version at the time of cache generation) and current
//!   fingerprint (version of currently provided data + current library version) matched, cached
//!   data is attempted to be loaded. If cache couldn't be loaded or fingerprints mismatch, EVE data
//!   is fetched and converted into adapted data. This process is relatively heavy on both IO and
//!   CPU, expect it to take approximately 1 second with locally sourced data. Adapted data is then
//!   fed to adapted data cacher implementation for caching and further converted for ease of access
//!   by the lib;
//! - you create [`SolarSystem`], specifying [`Src`] it should usee, and manipulate it to create
//!   fits with ships and items, and fetch various and stats.
//!
//! Three main entities available via [`SolarSystem`] are fleets, fits and items. Items come in many
//! kinds:
//!
//! - autocharges: charges which are automatically loaded into some item kinds by some effects, e.g.
//!   LR fighters have their bombs set as autocharges. Unlike regular charges, they expose effect ID
//!   which is using them, and cannot be removed;
//! - boosters: boosters expose slot they use and have side-effect-related methods;
//! - character: in EVE, character is an actual item which carries some attributes, and applies some
//!   modifications upon other items. Modern EVE seems to use just type ID 1373 for characters, but
//!   the lib does not set it automatically to every fit, you will have to add it yourself;
//! - charges: ammo, scripts and other things loadable into modules;
//! - drones: one of the more complex item kinds. Drones are separate "physic" objects, i.e. can
//!   have their own coordinate and movement set. Can be mutated, can have NPC prop mode set to
//!   control if they are using their pseudo-MWD or not (which affects their speed and signature
//!   radius);
//! - fighters: like drones, are "physic" objects. A fighter item represents a fighter squad, with
//!   ability to fetch squad count data, and override count. Fighters usually have multiple active
//!   abilities, which enable or disable effects on them;
//! - fit-wide effects: environmental effects which are assigned to a fit, and applied to all items
//!   of that fit;
//! - implants: just implant, the only special is that they expose slot they use under current EVE
//!   data;
//! - modules: like drones, one of more complex item kinds; can be mutated, have lots of extra
//!   methods which control how they cycle;
//! - projected effects: environmental effects which are applied only to specific items;
//! - rigs: ship rigs, nothing special about those;
//! - services: citadel/structure services like cloning, the only special thing is that they have
//!   their own set of states. FLEX structures also have services, which are usually automatically
//!   installed. In the lib, they are not automatically installed, you will have to add them
//!   yourself;
//! - ships: another "physic" object. Not much else is special about it, aside that all the items it
//!   carries (like modules) inherit its coordinates and movement;
//! - skills: character skill, assigned to a fit in this case. Skills have levels, and a single fit
//!   cannot have more than 2 skills with the same type ID. This is the only item which enforces
//!   strict type ID-based restriction;
//! - stances: t3 ship modes (e.g. t3 destroyers) are implemented as separate item, this is the item
//!   kind for that;
//! - subsystems: item kind to handle t3 ship subsystems; under the hood, subsystems take specific
//!   subsystem slot, and subsystems expose that (similar to booster/implant slots);
//! - system-wide effect: environmental effect which is applied to all items in the solar system.
//!
//! Most of those items are assigned directly to a fit, except for (auto)charges (which reside on
//! parent item) and system-wide/projected effects (which reside directly on solar system). Modules
//! are the only ones which are using ordered container.

extern crate core;

pub use api::{
    Ability, AbilityId, AbilityIter, AbilityMut, AddMode, Affector, AttrId, Autocharge, AutochargeMut, Booster,
    BoosterMut, Character, CharacterMut, Charge, ChargeMut, Coordinates, CtlAffectors, CustomAttrId, CustomEffectId,
    Direction, DogmaEffectId, Drone, DroneMut, EffectId, EffectiveMutation, EffectiveMutationMut, EveAttrId, Fighter,
    FighterMut, Fit, FitMut, Fleet, FleetMut, FullMAttr, FullMAttrIter, FullMAttrMut, FwEffect, FwEffectMut, Implant,
    ImplantMut, IncompleteMutation, IncompleteMutationMut, Item, ItemAttrValues, ItemCommon, ItemEffectInfo, ItemGrpId,
    ItemMut, ItemMutCommon, ItemTypeId, MinionState, Modification, Module, ModuleIter, ModuleMut, ModuleState,
    MoveMode, Movement, MutIter, Mutation, MutationMut, Op, Proj, ProjEffect, ProjEffectMut, ProjIter, ProjMut,
    ProjRange, RangedProj, RangedProjIter, RangedProjMut, RawMAttr, RawMAttrIter, RawMAttrMut, RemoveMode, Rig, RigMut,
    Service, ServiceMut, ServiceState, Ship, ShipMut, SideEffect, SideEffectIter, SideEffectMut, SideEffectPartialStr,
    SideEffectStr, Skill, SkillMut, Stance, StanceMut, Subsystem, SubsystemMut, SwEffect, SwEffectMut,
};
pub use def::VERSION;
pub use lender::Lender;
pub use misc::{
    BreacherProfile, DpsProfile, EffectMode, FighterCountInfo, ItemKind, ItemNpcPropInfo, ItemOptionalReloadInfo,
    ItemRearmMinionInfo, ItemSpoolInfo, ModRack, NpcProp, OptionExt, OptionalReload, RearmMinion, SecZone,
    SecZoneCorruption, Spool,
};
pub(crate) use misc::{DefOption, DefOptionExt};
pub use num::{Count, CountNz, FitSecStatus, Index, PValue, SkillLevel, SlotIndex, UnitInterval, Value};
pub use sol::SolarSystem;
pub use src::Src;
pub use ud::{FitId, FleetId, ItemId};

pub mod ad;
mod api;
mod dbg;
mod def;
pub mod ed;
pub mod err;
mod misc;
mod nd;
mod num;
mod rd;
mod sol;
pub mod src;
pub mod stats;
mod svc;
mod ud;
pub mod util;
pub mod val;
