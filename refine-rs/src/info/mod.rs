// Info commands are special in one regard: they use 2 separate formats to store backref-capable IDs
// and regular IDs. As such, they were not transformed into a generic form which could store either.
//
// Backref-capable commands use format which minimizes friction from used container types: plain
// vector is used.
//
// Regular ID ones use hashmap, because this format is the one consumed by info getters, and they
// need convenient access to the data. This saves one conversion (which can potentially involve many
// allocations, depending on override count).

pub(crate) use fit::FitInfoEnumCmd;
pub use fit::{
    FitGetFitInfoError, FitInfo, FitInfoCmd, FitInfoCmdBr, FitInfoEnumCmdBr, FitInfoEnumError, FitInfoExt, FitInfoMode,
};
pub use fleet::{FleetGetFleetInfoError, FleetInfo, FleetInfoCmd, FleetInfoCmdBr, FleetInfoExt, FleetInfoMode};
pub use item::{
    AbilityInfo, AttrMutationInfo, AutochargeInfo, AutochargeInfoExt, BoosterInfo, BoosterInfoExt, CharacterInfo,
    CharacterInfoExt, ChargeInfo, ChargeInfoExt, DroneInfo, DroneInfoExt, FighterInfo, FighterInfoExt, FwEffectInfo,
    FwEffectInfoExt, ImplantInfo, ImplantInfoExt, ItemGetItemInfoError, ItemInfo, ItemInfoCmd, ItemInfoCmdBr,
    ItemInfoMode, ItemMutationInfo, ModuleInfo, ModuleInfoExt, ProjEffectInfo, ProjEffectInfoExt, ProjInfo,
    RangedProjInfo, RigInfo, RigInfoExt, ServiceInfo, ServiceInfoExt, ShipInfo, ShipInfoExt, SideEffectInfo,
    SideEffectMod, SideEffectOp, SkillInfo, SkillInfoExt, StanceInfo, StanceInfoExt, SubsystemInfo, SubsystemInfoExt,
    SwEffectInfo, SwEffectInfoExt,
};
pub(crate) use sol::SolInfoEnumCmd;
pub use sol::{SolInfo, SolInfoCmd, SolInfoCmdBr, SolInfoEnumCmdBr, SolInfoEnumError, SolInfoExt, SolInfoMode};
pub use src::{SrcInfo, SrcInfoExt, SrcInfoMode};

mod fit;
mod fleet;
mod item;
mod sol;
mod src;
