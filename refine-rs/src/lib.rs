#![feature(default_field_values)]

pub use cmd::{
    AddFitCmd, AddFleetCmd, AddItemEnumCmd, AddSolCmd, AddedFitIdResp, AddedFleetIdResp, AddedItemIdsResp,
    ChangeFitEnumCmd, ChangeItemEnumCmd, ChangeSolEnumCmd, ChangedItemIdsResp, CmdResp, CmdResps, FitAddBoosterCmd,
    FitAddRigCmd, FitChangeAutochargeCmd, FitChangeBoosterCmd, FitChangeFitCmd, FitIdBackref, FitRemoveItemCmd,
    FleetIdBackref, ItemAddBoosterCmd, ItemAddRigCmd, ItemChangeAutochargeCmd, ItemChangeBoosterCmd, ItemIdBackref,
    RemoveFitCmd, RemoveFleetCmd, RemoveItemCmd, SolAddBoosterCmd, SolAddFitCmd, SolAddFleetCmd, SolAddRigCmd,
    SolChangeAutochargeCmd, SolChangeBoosterCmd, SolChangeFitCmd, SolChangeFleetCmd, SolChangeSolCmd, SolRemoveFitCmd,
    SolRemoveFleetCmd, SolRemoveItemCmd,
};
pub use fit::Fit;
pub use fleet::Fleet;
pub use info::{SrcInfo, SrcInfoMode};
pub use item::Item;
pub use rc::{
    BreacherProfile, Count, DpsProfile, EffectId, EffectMode, FitId, FleetId, ItemId, NpcProp, OptionalReload, PValue,
    RearmMinion, RmMode, SecZone, SecZoneCorruption, Spool, UnitInterval, Value, ed::EveDataHandler,
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
