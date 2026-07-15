#![feature(default_field_values)]

pub use cmd::{
    AddFitCmd, AddFleetCmd, AddItemEnumCmd, AddMutation, AddSolCmd, AddedFitIdResp, AddedFleetIdResp, AddedItemIdsResp,
    AttrMutation, ChangeFitEnumCmd, ChangeItemEnumCmd, ChangeMutation, ChangeSolEnumCmd, ChangedItemIdsResp, CmdResp,
    CmdResps, FitAddBoosterCmd, FitAddDroneCmd, FitAddFighterCmd, FitAddRigCmd, FitChangeAutochargeCmd,
    FitChangeBoosterCmd, FitChangeCharacterCmd, FitChangeChargeCmd, FitChangeDroneCmd, FitChangeFighterCmd,
    FitChangeFitCmd, FitChangeRigCmd, FitIdBackref, FitRemoveItemCmd, FitSetCharacterCmd, FitUnsetCharacterCmd,
    FleetIdBackref, ItemAddBoosterCmd, ItemAddDroneCmd, ItemAddFighterCmd, ItemAddRigCmd, ItemChangeAutochargeCmd,
    ItemChangeBoosterCmd, ItemChangeCharacterCmd, ItemChangeChargeCmd, ItemChangeDroneCmd, ItemChangeFighterCmd,
    ItemChangeRigCmd, ItemIdBackref, ItemSetCharacterCmd, RemoveFitCmd, RemoveFleetCmd, RemoveItemCmd,
    SolAddBoosterCmd, SolAddDroneCmd, SolAddFighterCmd, SolAddFitCmd, SolAddFleetCmd, SolAddRigCmd,
    SolChangeAutochargeCmd, SolChangeBoosterCmd, SolChangeCharacterCmd, SolChangeCharacterViaFitCmd,
    SolChangeCharacterViaItemCmd, SolChangeChargeCmd, SolChangeDroneCmd, SolChangeFighterCmd, SolChangeFitCmd,
    SolChangeFleetCmd, SolChangeRigCmd, SolChangeSolCmd, SolRemoveFitCmd, SolRemoveFleetCmd, SolRemoveItemCmd,
    SolSetCharacterCmd, SolUnsetCharacterCmd,
};
pub use fit::Fit;
pub use fleet::Fleet;
pub use info::{SrcInfo, SrcInfoMode};
pub use item::Item;
pub use rc::{
    BreacherProfile, Coordinates, Count, CountNz, DpsProfile, EffectId, EffectMode, FitId, FleetId, ItemId,
    MinionState, ModuleState, Movement, NpcProp, OptionalReload, PValue, RearmMinion, RmMode, SecZone,
    SecZoneCorruption, ServiceState, Spool, UnitInterval, Value, ed::EveDataHandler,
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
