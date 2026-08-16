#![feature(default_field_values)]
#![feature(never_type)]
#![feature(const_trait_impl)]
#![feature(const_default)]
#![cfg_attr(feature = "serde", feature(cfg_eval))]

pub use api::{Fit, Fleet, Item, Refine, SolarSystem, SolarSystemId};
pub use ctl::{
    AddFleetCmd, AddItemEnumCmd, AddMutation, AddSolCmd, AddedFitIdResp, AddedFleetIdResp, AddedItemIdsResp,
    AttrMutation, ChangeFleetCmd, ChangeItemEnumCmd, ChangeMutation, ChangedItemIdsResp, CtlCmdResp, CtlCmdResps,
    FitAddBoosterCmd, FitAddCmd, FitAddCmdBackref, FitAddDroneCmd, FitAddFighterCmd, FitAddFwEffectCmd,
    FitAddImplantCmd, FitAddModuleCmd, FitAddRigCmd, FitAddServiceCmd, FitAddSkillCmd, FitAddSubsystemCmd,
    FitChangeAutochargeCmd, FitChangeBoosterCmd, FitChangeCharacterCmd, FitChangeChargeCmd, FitChangeCmd,
    FitChangeCmdBackref, FitChangeDroneCmd, FitChangeFighterCmd, FitChangeFwEffectCmd, FitChangeImplantCmd,
    FitChangeModuleCmd, FitChangeRigCmd, FitChangeServiceCmd, FitChangeShipCmd, FitChangeSkillCmd, FitChangeStanceCmd,
    FitChangeSubsystemCmd, FitCtlCmd, FitIdBackref, FitRemoveCmd, FitRemoveItemCmd, FitSetCharacterCmd, FitSetShipCmd,
    FitSetStanceCmd, FitUnsetCharacterCmd, FitUnsetShipCmd, FitUnsetStanceCmd, FleetIdBackref, ItemAddBoosterCmd,
    ItemAddDroneCmd, ItemAddFighterCmd, ItemAddFwEffectCmd, ItemAddImplantCmd, ItemAddModuleCmd, ItemAddProjEffectCmd,
    ItemAddRigCmd, ItemAddServiceCmd, ItemAddSkillCmd, ItemAddSubsystemCmd, ItemAddSwEffectCmd,
    ItemChangeAutochargeCmd, ItemChangeBoosterCmd, ItemChangeCharacterCmd, ItemChangeChargeCmd, ItemChangeDroneCmd,
    ItemChangeFighterCmd, ItemChangeFwEffectCmd, ItemChangeImplantCmd, ItemChangeModuleCmd, ItemChangeProjEffectCmd,
    ItemChangeRigCmd, ItemChangeServiceCmd, ItemChangeShipCmd, ItemChangeSkillCmd, ItemChangeStanceCmd,
    ItemChangeSubsystemCmd, ItemChangeSwEffectCmd, ItemIdBackref, ItemSetCharacterCmd, ItemSetShipCmd,
    ItemSetStanceCmd, RemoveFleetCmd, RemoveItemCmd, SolAddBoosterCmd, SolAddDroneCmd, SolAddFighterCmd,
    SolAddFleetCmd, SolAddFwEffectCmd, SolAddImplantCmd, SolAddModuleCmd, SolAddProjEffectCmd, SolAddRigCmd,
    SolAddServiceCmd, SolAddSkillCmd, SolAddSubsystemCmd, SolAddSwEffectCmd, SolChangeAutochargeCmd,
    SolChangeBoosterCmd, SolChangeCharacterCmd, SolChangeCharacterViaFitCmd, SolChangeCharacterViaItemCmd,
    SolChangeChargeCmd, SolChangeDroneCmd, SolChangeFighterCmd, SolChangeFleetCmd, SolChangeFwEffectCmd,
    SolChangeImplantCmd, SolChangeModuleCmd, SolChangeProjEffectCmd, SolChangeRigCmd, SolChangeServiceCmd,
    SolChangeShipCmd, SolChangeShipViaFitCmd, SolChangeShipViaItemCmd, SolChangeSkillCmd, SolChangeSolCmd,
    SolChangeStanceCmd, SolChangeStanceViaFitCmd, SolChangeStanceViaItemCmd, SolChangeSubsystemCmd,
    SolChangeSwEffectCmd, SolCtlCmd, SolRemoveFleetCmd, SolRemoveItemCmd, SolSetCharacterCmd, SolSetShipCmd,
    SolSetStanceCmd, SolUnsetCharacterCmd, SolUnsetShipCmd, SolUnsetStanceCmd,
};
pub use info::{
    AbilityInfo, AttrMutationInfo, AutochargeInfo, AutochargeInfoExt, BoosterInfo, BoosterInfoExt, CharacterInfo,
    CharacterInfoExt, ChargeInfo, ChargeInfoExt, DroneInfo, DroneInfoExt, FighterInfo, FighterInfoExt, FitInfo,
    FitInfoCmd, FitInfoCmdBackref, FitInfoExt, FitInfoMode, FleetInfo, FleetInfoCmd, FleetInfoExt, FleetInfoMode,
    FwEffectInfo, FwEffectInfoExt, ImplantInfo, ImplantInfoExt, ItemInfo, ItemInfoCmd, ItemInfoMode, ItemMutationInfo,
    ModuleInfo, ModuleInfoExt, ProjEffectInfo, ProjEffectInfoExt, ProjInfo, RangedProjInfo, RigInfo, RigInfoExt,
    ServiceInfo, ServiceInfoExt, ShipInfo, ShipInfoExt, SideEffectInfo, SideEffectMod, SideEffectOp, SkillInfo,
    SkillInfoExt, SolInfo, SolInfoCmd, SolInfoCmdBackref, SolInfoExt, SolInfoMode, StanceInfo, StanceInfoExt,
    SubsystemInfo, SubsystemInfoExt, SwEffectInfo, SwEffectInfoExt,
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
