#![feature(default_field_values)]

pub use cmd::{
    ChangeFitEnumCmd, ChangeItemEnumCmd, ChangeSolEnumCmd, ChangedItemIdsResp, CmdResp, CmdResps, CreateFitCmd,
    CreateFleetCmd, CreateItemEnumCmd, CreateSolCmd, CreatedFitIdResp, CreatedFleetIdResp, CreatedItemIdsResp,
    FitChangeAutochargeCmd, FitChangeFitCmd, FitIdBackref, FitRemoveItemCmd, FleetIdBackref, ItemChangeAutochargeCmd,
    ItemCreateRigCmd, ItemIdBackref, RemoveFitCmd, RemoveFleetCmd, RemoveItemCmd, SolChangeAutochargeCmd,
    SolChangeFitCmd, SolChangeFleetCmd, SolChangeSolCmd, SolCreateFitCmd, SolCreateFleetCmd, SolRemoveFitCmd,
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
