#![feature(default_field_values)]
#![feature(never_type)]
#![feature(const_trait_impl)]
#![feature(const_default)]
#![cfg_attr(feature = "serde", feature(cfg_eval))]

pub use api::{Fit, Fleet, Item, Refine, SolarSystem, SolarSystemId};
pub use ctl::{
    AddMutation, AttrMutation, AutochargeChangeCmd, BoosterAddCmd, BoosterChangeCmd, ChangeMutation,
    CharacterChangeCmd, CharacterSetCmd, CharacterUnsetCmd, ChargeChangeCmd, DroneAddCmd, DroneAddCmdBr,
    DroneChangeCmd, DroneChangeCmdBr, FighterAddCmd, FighterAddCmdBr, FighterChangeCmd, FighterChangeCmdBr, FitAddCmd,
    FitAddCmdBr, FitChangeCmd, FitChangeCmdBr, FitChangeEnumCmd, FitChangeEnumCmdBr, FitRemoveCmd, FleetAddCmd,
    FleetAddCmdBr, FleetChangeCmd, FleetChangeCmdBr, FleetRemoveCmd, FwEffectAddCmd, FwEffectChangeCmd, ImplantAddCmd,
    ImplantChangeCmd, ItemAddEnumCmd, ItemChangeEnumCmd, ItemRemoveCmd, ModuleAddCmd, ModuleAddCmdBr, ModuleChangeCmd,
    ModuleChangeCmdBr, ProjEffectAddCmd, ProjEffectAddCmdBr, ProjEffectChangeCmd, ProjEffectChangeCmdBr, RigAddCmd,
    RigChangeCmd, ServiceAddCmd, ServiceChangeCmd, ShipChangeCmd, ShipSetCmd, ShipUnsetCmd, SkillAddCmd,
    SkillChangeCmd, SolAddCmd, SolChangeCmd, SolChangeEnumCmd, SolChangeEnumCmdBr, StanceChangeCmd, StanceSetCmd,
    StanceUnsetCmd, SubsystemAddCmd, SubsystemChangeCmd, SwEffectAddCmd, SwEffectChangeCmd,
};
pub use hyb::{FitHybridCmdBr, SolHybridCmdBr};
pub use info::{
    AbilityInfo, AttrMutationInfo, AutochargeInfo, AutochargeInfoExt, BoosterInfo, BoosterInfoExt, CharacterInfo,
    CharacterInfoExt, ChargeInfo, ChargeInfoExt, DroneInfo, DroneInfoExt, FighterInfo, FighterInfoExt, FitInfo,
    FitInfoCmd, FitInfoCmdBr, FitInfoEnumCmdBr, FitInfoExt, FitInfoMode, FleetInfo, FleetInfoCmd, FleetInfoExt,
    FleetInfoMode, FwEffectInfo, FwEffectInfoExt, ImplantInfo, ImplantInfoExt, ItemInfo, ItemInfoCmd, ItemInfoCmdBr,
    ItemInfoMode, ItemMutationInfo, ModuleInfo, ModuleInfoExt, ProjEffectInfo, ProjEffectInfoExt, ProjInfo,
    RangedProjInfo, RigInfo, RigInfoExt, ServiceInfo, ServiceInfoExt, ShipInfo, ShipInfoExt, SideEffectInfo,
    SideEffectMod, SideEffectOp, SkillInfo, SkillInfoExt, SolInfo, SolInfoCmd, SolInfoCmdBr, SolInfoEnumCmdBr,
    SolInfoExt, SolInfoMode, StanceInfo, StanceInfoExt, SubsystemInfo, SubsystemInfoExt, SwEffectInfo, SwEffectInfoExt,
};
pub use rc::{
    AbilityId, AddMode, Affector, AttrId, BreacherProfile, Coordinates, Count, CountNz, DefOption, DefOptionExt,
    Direction, DpsProfile, EffectId, EffectMode, FighterCountInfo, FitId, FitSecStatus, FleetId, Index, ItemAttrValues,
    ItemEffectInfo, ItemGrpId, ItemId, ItemKind, ItemNpcPropInfo, ItemOptionalReloadInfo, ItemRearmMinionInfo,
    ItemSpoolInfo, ItemTypeId, MinionState, ModRack, Modification, ModuleState, MoveMode, Movement, NpcProp, Op,
    OptionExt, OptionalReload, PValue, ProjRange, RearmMinion, RemoveMode, SecZone, SecZoneCorruption, ServiceState,
    SkillLevel, SlotIndex, Spool, UnitInterval, Value, ad::AdaptedDataCacher, ed::EveDataHandler,
};
pub use shared::{
    AddedFitIdResp, AddedFleetIdResp, AddedItemIdsResp, ChangedItemIdsResp, CmdResp, CmdResps, FitIdBr, FleetIdBr,
    ItemIdBr, TriStateField,
};
pub use src::{Src, SrcAlias};

mod api;
mod ctl;
pub mod dev;
pub mod err;
mod hyb;
mod info;
mod shared;
pub mod src;
pub mod stats;
mod svc;
pub mod trial;
pub mod val;
