#![feature(default_field_values)]
#![feature(const_trait_impl)]
#![feature(const_convert)]
#![feature(const_default)]
#![feature(const_destruct)]
#![cfg_attr(feature = "serde", feature(cfg_eval))]
#![expect(unevaluated_default_field_value)]

pub use api::{Fit, Fleet, Item, Refine, Sol, SolId, Src, SrcAlias};
pub use misc::{CmdResp, CmdResps, FitIdBr, FleetIdBr, ItemIdBr, TriStateField};
pub use rc::{
    AbilityId, AddMode, Affector, AttrId, BreacherProfile, Coordinates, Count, CountNz, CustomAttrId, CustomEffectId,
    DetectedItemKind, Direction, DogmaEffectId, DpsProfile, EffectId, EffectMode, EveAttrId, FighterCountInfo, FitId,
    FitSecStatus, FleetId, ItemAttrValues, ItemEffectInfo, ItemGrpId, ItemId, ItemKind, ItemSpoolInfo, ItemTypeId,
    MinionState, ModRack, Modification, ModuleState, MoveMode, Movement, NpcProp, Op, OptionExt, OptionalReload,
    PValue, ProjRange, RackPos, RearmMinion, RemoveMode, SecZone, SecZoneCorruption, ServiceState, SkillLevel,
    SlotIndex, Spool, SpoolCycleInfo, UnitInterval, Value,
};

mod api;
pub mod app;
pub mod ctl;
#[cfg(feature = "dev")]
pub mod dev;
pub mod err;
pub mod hyb;
pub mod info;
mod misc;
pub mod src;
pub mod stats;
mod svc;
pub mod trial;
mod util;
pub mod val;
