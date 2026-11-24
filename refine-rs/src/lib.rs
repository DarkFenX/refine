#![feature(default_field_values)]
#![feature(const_trait_impl)]
#![feature(const_convert)]
#![feature(const_default)]
#![feature(const_destruct)]
#![cfg_attr(feature = "serde", feature(cfg_eval))]
#![expect(unevaluated_default_field_value)]

pub use api::{Fit, Fleet, Item, Refine, Sol, SolId};
pub use ctl::{
    AddMutation, AddedFitIdResp, AddedFleetIdResp, AddedItemIdsResp, AttrMutation, AutochargeChangeCmd,
    AutochargeItemIds, BoosterAddCmd, BoosterChangeCmd, ChangeMutation, ChangedItemIdsResp, CharacterChangeCmd,
    CharacterSetCmd, CharacterUnsetCmd, ChargeChangeCmd, DroneAddCmd, DroneAddCmdBr, DroneChangeCmd, DroneChangeCmdBr,
    FighterAddCmd, FighterAddCmdBr, FighterChangeCmd, FighterChangeCmdBr, FitAddCmd, FitAddCmdBr, FitChangeCmd,
    FitChangeCmdBr, FitChangeEnumCmd, FitChangeEnumCmdBr, FitRemoveCmd, FleetAddCmd, FleetAddCmdBr, FleetChangeCmd,
    FleetChangeCmdBr, FleetRemoveCmd, FwEffectAddCmd, FwEffectChangeCmd, ImplantAddCmd, ImplantChangeCmd,
    ItemAddEnumCmd, ItemAutodetectAddCmd, ItemChangeEnumCmd, ItemRemoveCmd, ModuleAddCmd, ModuleAddCmdBr,
    ModuleChangeCmd, ModuleChangeCmdBr, ProjEffectAddCmd, ProjEffectAddCmdBr, ProjEffectChangeCmd,
    ProjEffectChangeCmdBr, RigAddCmd, RigChangeCmd, ServiceAddCmd, ServiceChangeCmd, ShipChangeCmd, ShipSetCmd,
    ShipUnsetCmd, SkillAddCmd, SkillChangeCmd, SolAddCmd, SolChangeCmd, SolChangeEnumCmd, SolChangeEnumCmdBr,
    StanceChangeCmd, StanceSetCmd, StanceUnsetCmd, SubsystemAddCmd, SubsystemChangeCmd, SwEffectAddCmd,
    SwEffectChangeCmd,
};
pub use hyb::{FitHybridCmdBr, SolHybridCmdBr};
pub use info::{
    AbilityInfo, AttrMutationInfo, AutochargeInfo, AutochargeInfoPartial, BoosterInfo, BoosterInfoPartial,
    CharacterInfo, CharacterInfoPartial, ChargeInfo, ChargeInfoPartial, DroneInfo, DroneInfoPartial, FighterInfo,
    FighterInfoPartial, FitInfo, FitInfoCmd, FitInfoCmdBr, FitInfoEnumCmdBr, FitInfoFull, FitInfoMode, FleetInfo,
    FleetInfoCmd, FleetInfoCmdBr, FleetInfoFull, FleetInfoMode, FwEffectInfo, FwEffectInfoPartial, ImplantInfo,
    ImplantInfoPartial, ItemInfo, ItemInfoCmd, ItemInfoCmdBr, ItemInfoFull, ItemInfoMode, ItemMutationDormantInfo,
    ItemMutationEffectiveInfo, ItemMutationInfo, ModuleInfo, ModuleInfoPartial, ModuleRacks, ProjEffectInfo,
    ProjEffectInfoPartial, ProjInfo, RangedProjInfo, RefineInfo, RefineInfoCmd, RigInfo, RigInfoPartial, ServiceInfo,
    ServiceInfoPartial, ShipInfo, ShipInfoPartial, SideEffectInfo, SideEffectMod, SideEffectOp, SkillInfo,
    SkillInfoPartial, SolInfo, SolInfoCmd, SolInfoCmdBr, SolInfoEnumCmdBr, SolInfoFull, SolInfoMode, SolInfoPartial,
    StanceInfo, StanceInfoPartial, SubsystemInfo, SubsystemInfoPartial, SwEffectInfo, SwEffectInfoPartial,
};
pub use rc::{
    AbilityId, AddMode, Affector, AttrId, BreacherProfile, Coordinates, Count, CountNz, CustomAttrId, CustomEffectId,
    DetectedItemKind, Direction, DogmaEffectId, DpsProfile, EffectId, EffectMode, EveAttrId, FighterCountInfo, FitId,
    FitSecStatus, FleetId, ItemAttrValues, ItemEffectInfo, ItemGrpId, ItemId, ItemKind, ItemSpoolInfo, ItemTypeId,
    MinionState, ModRack, Modification, ModuleState, MoveMode, Movement, NpcProp, Op, OptionExt, OptionalReload,
    PValue, ProjRange, RackPos, RearmMinion, RemoveMode, SecZone, SecZoneCorruption, ServiceState, SkillLevel,
    SlotIndex, Spool, SpoolCycleInfo, UnitInterval, Value, ad::AdaptedDataCacher, ed::EveDataHandler,
};
pub use shared::{CmdResp, CmdResps, FitIdBr, FleetIdBr, ItemIdBr, TriStateField};
pub use src::{Src, SrcAlias};

mod api;
pub mod app;
mod ctl;
#[cfg(feature = "dev")]
pub mod dev;
pub mod err;
mod hyb;
mod info;
mod shared;
pub mod src;
pub mod stats;
mod svc;
pub mod trial;
mod util;
pub mod val;
