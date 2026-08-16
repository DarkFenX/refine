#![feature(default_field_values)]
#![feature(never_type)]
#![feature(const_trait_impl)]
#![feature(const_default)]
#![cfg_attr(feature = "serde", feature(cfg_eval))]

pub use api::{Fit, Fleet, Item, Refine, SolarSystem, SolarSystemId};
pub use ctl::{
    AddMutation, AddedFitIdResp, AddedFleetIdResp, AddedItemIdsResp, AttrMutation, AutochargeChangeCmd, BoosterAddCmd,
    BoosterChangeCmd, ChangeMutation, ChangedItemIdsResp, ChargeChangeCmd, CtlCmdResp, CtlCmdResps, FitAddCmd,
    FitAddCmdBr, FitAddDroneCmd, FitAddFighterCmd, FitAddFwEffectCmd, FitAddModuleCmd, FitAddServiceCmd,
    FitAddSkillCmd, FitChangeCharacterCmd, FitChangeCmd, FitChangeCmdBr, FitChangeDroneCmd, FitChangeFighterCmd,
    FitChangeFwEffectCmd, FitChangeModuleCmd, FitChangeServiceCmd, FitChangeShipCmd, FitChangeSkillCmd,
    FitChangeStanceCmd, FitCtlCmd, FitIdBr, FitRemoveCmd, FitSetCharacterCmd, FitSetShipCmd, FitSetStanceCmd,
    FitUnsetCharacterCmd, FitUnsetShipCmd, FitUnsetStanceCmd, FleetAddCmd, FleetAddCmdBr, FleetChangeCmd,
    FleetChangeCmdBr, FleetIdBr, FleetRemoveCmd, ImplantAddCmd, ImplantChangeCmd, ItemAddCmd, ItemAddDroneCmd,
    ItemAddFighterCmd, ItemAddFwEffectCmd, ItemAddModuleCmd, ItemAddProjEffectCmd, ItemAddServiceCmd, ItemAddSkillCmd,
    ItemAddSwEffectCmd, ItemChangeCharacterCmd, ItemChangeDroneCmd, ItemChangeFighterCmd, ItemChangeFwEffectCmd,
    ItemChangeModuleCmd, ItemChangeProjEffectCmd, ItemChangeServiceCmd, ItemChangeShipCmd, ItemChangeSkillCmd,
    ItemChangeStanceCmd, ItemChangeSwEffectCmd, ItemCtlCmd, ItemIdBr, ItemRemoveCmd, ItemSetCharacterCmd,
    ItemSetShipCmd, ItemSetStanceCmd, RigAddCmd, RigChangeCmd, SolAddCmd, SolAddDroneCmd, SolAddFighterCmd,
    SolAddFwEffectCmd, SolAddModuleCmd, SolAddProjEffectCmd, SolAddServiceCmd, SolAddSkillCmd, SolAddSwEffectCmd,
    SolChangeCharacterCmd, SolChangeCharacterViaFitCmd, SolChangeCharacterViaItemCmd, SolChangeCmd, SolChangeDroneCmd,
    SolChangeFighterCmd, SolChangeFwEffectCmd, SolChangeModuleCmd, SolChangeProjEffectCmd, SolChangeServiceCmd,
    SolChangeShipCmd, SolChangeShipViaFitCmd, SolChangeShipViaItemCmd, SolChangeSkillCmd, SolChangeStanceCmd,
    SolChangeStanceViaFitCmd, SolChangeStanceViaItemCmd, SolChangeSwEffectCmd, SolCtlCmd, SolSetCharacterCmd,
    SolSetShipCmd, SolSetStanceCmd, SolUnsetCharacterCmd, SolUnsetShipCmd, SolUnsetStanceCmd, SubsystemAddCmd,
    SubsystemChangeCmd,
};
pub use info::{
    AbilityInfo, AttrMutationInfo, AutochargeInfo, AutochargeInfoExt, BoosterInfo, BoosterInfoExt, CharacterInfo,
    CharacterInfoExt, ChargeInfo, ChargeInfoExt, DroneInfo, DroneInfoExt, FighterInfo, FighterInfoExt, FitInfo,
    FitInfoCmd, FitInfoCmdBr, FitInfoExt, FitInfoMode, FleetInfo, FleetInfoCmd, FleetInfoExt, FleetInfoMode,
    FwEffectInfo, FwEffectInfoExt, ImplantInfo, ImplantInfoExt, ItemInfo, ItemInfoCmd, ItemInfoMode, ItemMutationInfo,
    ModuleInfo, ModuleInfoExt, ProjEffectInfo, ProjEffectInfoExt, ProjInfo, RangedProjInfo, RigInfo, RigInfoExt,
    ServiceInfo, ServiceInfoExt, ShipInfo, ShipInfoExt, SideEffectInfo, SideEffectMod, SideEffectOp, SkillInfo,
    SkillInfoExt, SolInfo, SolInfoCmd, SolInfoCmdBr, SolInfoExt, SolInfoMode, StanceInfo, StanceInfoExt, SubsystemInfo,
    SubsystemInfoExt, SwEffectInfo, SwEffectInfoExt,
};
pub use rc::{
    AbilityId, AddMode, Affector, AttrId, BreacherProfile, Coordinates, Count, CountNz, DefOption, DefOptionExt,
    Direction, DpsProfile, EffectId, EffectMode, FighterCountInfo, FitId, FitSecStatus, FleetId, Index, ItemAttrValues,
    ItemEffectInfo, ItemGrpId, ItemId, ItemKind, ItemNpcPropInfo, ItemOptionalReloadInfo, ItemRearmMinionInfo,
    ItemSpoolInfo, ItemTypeId, MinionState, ModRack, Modification, ModuleState, MoveMode, Movement, NpcProp, Op,
    OptionalReload, PValue, ProjRange, RearmMinion, RemoveMode, SecZone, SecZoneCorruption, ServiceState, SkillLevel,
    SlotIndex, Spool, UnitInterval, Value, ad::AdaptedDataCacher, ed::EveDataHandler,
};
pub use src::{Src, SrcAlias};
pub use util::TriStateField;

mod api;
mod ctl;
pub mod dev;
pub mod err;
mod info;
pub mod src;
pub mod stats;
mod svc;
mod util;
pub mod val;
