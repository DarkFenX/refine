#![feature(default_field_values)]

pub use api::{Fit, Fleet, Item, Refine, SolarSystem, SolarSystemId};
pub use cmd::{
    AddFitCmd, AddFleetCmd, AddItemEnumCmd, AddMutation, AddSolCmd, AddedFitIdResp, AddedFleetIdResp, AddedItemIdsResp,
    AttrMutation, ChangeFitEnumCmd, ChangeFleetCmd, ChangeItemEnumCmd, ChangeMutation, ChangeSolEnumCmd,
    ChangedItemIdsResp, CmdResp, CmdResps, FitAddBoosterCmd, FitAddDroneCmd, FitAddFighterCmd, FitAddFwEffectCmd,
    FitAddImplantCmd, FitAddModuleCmd, FitAddRigCmd, FitAddServiceCmd, FitAddSkillCmd, FitAddSubsystemCmd,
    FitChangeAutochargeCmd, FitChangeBoosterCmd, FitChangeCharacterCmd, FitChangeChargeCmd, FitChangeDroneCmd,
    FitChangeFighterCmd, FitChangeFitCmd, FitChangeFwEffectCmd, FitChangeImplantCmd, FitChangeModuleCmd,
    FitChangeRigCmd, FitChangeServiceCmd, FitChangeShipCmd, FitChangeSkillCmd, FitChangeStanceCmd,
    FitChangeSubsystemCmd, FitIdBackref, FitRemoveItemCmd, FitSetCharacterCmd, FitSetShipCmd, FitSetStanceCmd,
    FitUnsetCharacterCmd, FitUnsetShipCmd, FitUnsetStanceCmd, FleetIdBackref, ItemAddBoosterCmd, ItemAddDroneCmd,
    ItemAddFighterCmd, ItemAddFwEffectCmd, ItemAddImplantCmd, ItemAddModuleCmd, ItemAddProjEffectCmd, ItemAddRigCmd,
    ItemAddServiceCmd, ItemAddSkillCmd, ItemAddSubsystemCmd, ItemAddSwEffectCmd, ItemChangeAutochargeCmd,
    ItemChangeBoosterCmd, ItemChangeCharacterCmd, ItemChangeChargeCmd, ItemChangeDroneCmd, ItemChangeFighterCmd,
    ItemChangeFwEffectCmd, ItemChangeImplantCmd, ItemChangeModuleCmd, ItemChangeProjEffectCmd, ItemChangeRigCmd,
    ItemChangeServiceCmd, ItemChangeShipCmd, ItemChangeSkillCmd, ItemChangeStanceCmd, ItemChangeSubsystemCmd,
    ItemChangeSwEffectCmd, ItemIdBackref, ItemSetCharacterCmd, ItemSetShipCmd, ItemSetStanceCmd, RemoveFitCmd,
    RemoveFleetCmd, RemoveItemCmd, SolAddBoosterCmd, SolAddDroneCmd, SolAddFighterCmd, SolAddFitCmd, SolAddFleetCmd,
    SolAddFwEffectCmd, SolAddImplantCmd, SolAddModuleCmd, SolAddProjEffectCmd, SolAddRigCmd, SolAddServiceCmd,
    SolAddSkillCmd, SolAddSubsystemCmd, SolAddSwEffectCmd, SolChangeAutochargeCmd, SolChangeBoosterCmd,
    SolChangeCharacterCmd, SolChangeCharacterViaFitCmd, SolChangeCharacterViaItemCmd, SolChangeChargeCmd,
    SolChangeDroneCmd, SolChangeFighterCmd, SolChangeFitCmd, SolChangeFleetCmd, SolChangeFwEffectCmd,
    SolChangeImplantCmd, SolChangeModuleCmd, SolChangeProjEffectCmd, SolChangeRigCmd, SolChangeServiceCmd,
    SolChangeShipCmd, SolChangeShipViaFitCmd, SolChangeShipViaItemCmd, SolChangeSkillCmd, SolChangeSolCmd,
    SolChangeStanceCmd, SolChangeStanceViaFitCmd, SolChangeStanceViaItemCmd, SolChangeSubsystemCmd,
    SolChangeSwEffectCmd, SolRemoveFitCmd, SolRemoveFleetCmd, SolRemoveItemCmd, SolSetCharacterCmd, SolSetShipCmd,
    SolSetStanceCmd, SolUnsetCharacterCmd, SolUnsetShipCmd, SolUnsetStanceCmd,
};
// TODO: consider moving src-related exports to its own module (check core lib for consistency)
pub use info::{
    AbilityInfo, AttrMutationInfo, AutochargeInfo, AutochargeInfoExt, BoosterInfo, BoosterInfoExt, CharacterInfo,
    CharacterInfoExt, ChargeInfo, ChargeInfoExt, DroneInfo, DroneInfoExt, FighterInfo, FighterInfoExt, FitInfo,
    FitInfoExt, FitInfoMode, FleetInfo, FleetInfoExt, FleetInfoMode, FwEffectInfo, FwEffectInfoExt, ImplantInfo,
    ImplantInfoExt, ItemInfo, ItemInfoMode, ItemMutationInfo, ModuleInfo, ModuleInfoExt, ProjEffectInfo,
    ProjEffectInfoExt, ProjInfo, RangedProjInfo, RigInfo, RigInfoExt, ServiceInfo, ServiceInfoExt, ShipInfo,
    ShipInfoExt, SideEffectInfo, SideEffectMod, SideEffectOp, SkillInfo, SkillInfoExt, SolInfo, SolInfoExt,
    SolInfoMode, StanceInfo, StanceInfoExt, SubsystemInfo, SubsystemInfoExt, SwEffectInfo, SwEffectInfoExt,
};
pub use rc::{
    AbilityId, AddMode, Affector, AttrId, AttrVals, BreacherProfile, Coordinates, Count, CountNz, Direction,
    DpsProfile, EffectId, EffectInfo, EffectMode, FighterCountInfo, FitId, FitSecStatus, FleetId, Index, ItemId,
    ItemKind, ItemNpcPropInfo, ItemOptionalReloadInfo, ItemRearmMinionInfo, ItemSpoolInfo, ItemTypeId, MinionState,
    ModRack, Modification, ModuleState, MoveMode, Movement, NpcProp, Op, OptionalReload, PValue, ProjRange,
    RearmMinion, RemoveMode, SecZone, SecZoneCorruption, ServiceState, SkillLevel, SlotIndex, Spool, UnitInterval,
    Value, ed::EveDataHandler,
};
pub use util::TriStateField;

mod api;
mod cmd;
pub mod dev;
pub mod err;
mod info;
pub mod src;
mod stats;
mod svc;
mod util;
pub mod val;
