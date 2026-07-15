#![feature(default_field_values)]

pub use cmd::{
    AddFitCmd, AddFleetCmd, AddItemEnumCmd, AddMutation, AddSolCmd, AddedFitIdResp, AddedFleetIdResp, AddedItemIdsResp,
    AttrMutation, ChangeFitEnumCmd, ChangeItemEnumCmd, ChangeMutation, ChangeSolEnumCmd, ChangedItemIdsResp, CmdResp,
    CmdResps, FitAddBoosterCmd, FitAddDroneCmd, FitAddRigCmd, FitChangeAutochargeCmd, FitChangeBoosterCmd,
    FitChangeCharacterCmd, FitChangeChargeCmd, FitChangeDroneCmd, FitChangeFitCmd, FitIdBackref, FitRemoveItemCmd,
    FitSetCharacterCmd, FitUnsetCharacterCmd, FleetIdBackref, ItemAddBoosterCmd, ItemAddDroneCmd, ItemAddRigCmd,
    ItemChangeAutochargeCmd, ItemChangeBoosterCmd, ItemChangeCharacterCmd, ItemChangeChargeCmd, ItemChangeDroneCmd,
    ItemIdBackref, ItemSetCharacterCmd, RemoveFitCmd, RemoveFleetCmd, RemoveItemCmd, SolAddBoosterCmd, SolAddDroneCmd,
    SolAddFitCmd, SolAddFleetCmd, SolAddRigCmd, SolChangeAutochargeCmd, SolChangeBoosterCmd, SolChangeCharacterCmd,
    SolChangeCharacterViaFitCmd, SolChangeCharacterViaItemCmd, SolChangeChargeCmd, SolChangeDroneCmd, SolChangeFitCmd,
    SolChangeFleetCmd, SolChangeSolCmd, SolRemoveFitCmd, SolRemoveFleetCmd, SolRemoveItemCmd, SolSetCharacterCmd,
    SolUnsetCharacterCmd,
};
pub use fit::Fit;
pub use fleet::Fleet;
pub use info::{SrcInfo, SrcInfoMode};
pub use item::Item;
pub use rc::{
    BreacherProfile, Coordinates, Count, DpsProfile, EffectId, EffectMode, FitId, FleetId, ItemId, MinionState,
    ModuleState, Movement, NpcProp, OptionalReload, PValue, RearmMinion, RmMode, SecZone, SecZoneCorruption,
    ServiceState, Spool, UnitInterval, Value, ed::EveDataHandler,
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
