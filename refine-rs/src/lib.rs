#![feature(default_field_values)]

pub use cmd::{
    AddFitCmd, AddFleetCmd, AddItemEnumCmd, AddMutation, AddSolCmd, AddedFitIdResp, AddedFleetIdResp, AddedItemIdsResp,
    AttrMutation, ChangeFitEnumCmd, ChangeItemEnumCmd, ChangeMutation, ChangeSolEnumCmd, ChangedItemIdsResp, CmdResp,
    CmdResps, FitAddBoosterCmd, FitAddDroneCmd, FitAddFighterCmd, FitAddFwEffectCmd, FitAddImplantCmd, FitAddModuleCmd,
    FitAddRigCmd, FitAddServiceCmd, FitChangeAutochargeCmd, FitChangeBoosterCmd, FitChangeCharacterCmd,
    FitChangeChargeCmd, FitChangeDroneCmd, FitChangeFighterCmd, FitChangeFitCmd, FitChangeFwEffectCmd,
    FitChangeImplantCmd, FitChangeModuleCmd, FitChangeRigCmd, FitChangeServiceCmd, FitIdBackref, FitRemoveItemCmd,
    FitSetCharacterCmd, FitUnsetCharacterCmd, FleetIdBackref, ItemAddBoosterCmd, ItemAddDroneCmd, ItemAddFighterCmd,
    ItemAddFwEffectCmd, ItemAddImplantCmd, ItemAddModuleCmd, ItemAddProjEffectCmd, ItemAddRigCmd, ItemAddServiceCmd,
    ItemChangeAutochargeCmd, ItemChangeBoosterCmd, ItemChangeCharacterCmd, ItemChangeChargeCmd, ItemChangeDroneCmd,
    ItemChangeFighterCmd, ItemChangeFwEffectCmd, ItemChangeImplantCmd, ItemChangeModuleCmd, ItemChangeProjEffectCmd,
    ItemChangeRigCmd, ItemChangeServiceCmd, ItemIdBackref, ItemSetCharacterCmd, RemoveFitCmd, RemoveFleetCmd,
    RemoveItemCmd, SolAddBoosterCmd, SolAddDroneCmd, SolAddFighterCmd, SolAddFitCmd, SolAddFleetCmd, SolAddFwEffectCmd,
    SolAddImplantCmd, SolAddModuleCmd, SolAddProjEffectCmd, SolAddRigCmd, SolAddServiceCmd, SolChangeAutochargeCmd,
    SolChangeBoosterCmd, SolChangeCharacterCmd, SolChangeCharacterViaFitCmd, SolChangeCharacterViaItemCmd,
    SolChangeChargeCmd, SolChangeDroneCmd, SolChangeFighterCmd, SolChangeFitCmd, SolChangeFleetCmd,
    SolChangeFwEffectCmd, SolChangeImplantCmd, SolChangeModuleCmd, SolChangeProjEffectCmd, SolChangeRigCmd,
    SolChangeServiceCmd, SolChangeSolCmd, SolRemoveFitCmd, SolRemoveFleetCmd, SolRemoveItemCmd, SolSetCharacterCmd,
    SolUnsetCharacterCmd,
};
pub use fit::Fit;
pub use fleet::Fleet;
pub use info::{SrcInfo, SrcInfoMode};
pub use item::Item;
pub use rc::{
    AddMode, BreacherProfile, Coordinates, Count, CountNz, DpsProfile, EffectId, EffectMode, FitId, FleetId, ItemId,
    MinionState, ModRack, ModuleState, MoveMode, Movement, NpcProp, OptionalReload, PValue, RearmMinion, RemoveMode,
    SecZone, SecZoneCorruption, ServiceState, Spool, UnitInterval, Value, ed::EveDataHandler,
};
pub use refine::Refine;
pub use sol::{SolarSystem, SolarSystemId};
pub use src::{Src, SrcAlias};

mod cmd;
pub mod err;
mod fit;
mod fleet;
mod info;
mod item;
mod refine;
mod sol;
mod src;
mod util;
