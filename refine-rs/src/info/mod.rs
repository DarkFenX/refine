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
    FitGetFitInfoError, FitInfo, FitInfoCmd, FitInfoCmdBr, FitInfoEnumCmdBr, FitInfoEnumError, FitInfoFull,
    FitInfoMode, ModuleRacks,
};
pub use fleet::{FleetGetFleetInfoError, FleetInfo, FleetInfoCmd, FleetInfoCmdBr, FleetInfoFull, FleetInfoMode};
pub use item::{
    AbilityInfo, AttrMutationInfo, AutochargeInfo, AutochargeInfoPartial, BoosterInfo, BoosterInfoPartial,
    CharacterInfo, CharacterInfoPartial, ChargeInfo, ChargeInfoPartial, DroneInfo, DroneInfoPartial, FighterInfo,
    FighterInfoPartial, FitItemGetItemInfoError, FwEffectInfo, FwEffectInfoPartial, ImplantInfo, ImplantInfoPartial,
    ItemGetItemInfoError, ItemInfo, ItemInfoCmd, ItemInfoCmdBr, ItemInfoFull, ItemInfoMode, ItemMutationDormantInfo,
    ItemMutationEffectiveInfo, ItemMutationInfo, ModuleInfo, ModuleInfoPartial, ProjEffectInfo, ProjEffectInfoPartial,
    ProjInfo, RangedProjInfo, RigInfo, RigInfoPartial, ServiceInfo, ServiceInfoPartial, ShipInfo, ShipInfoPartial,
    SideEffectInfo, SideEffectMod, SideEffectOp, SkillInfo, SkillInfoPartial, StanceInfo, StanceInfoPartial,
    SubsystemInfo, SubsystemInfoPartial, SwEffectInfo, SwEffectInfoPartial,
};
pub use refine::{RefineInfo, RefineInfoCmd};
pub(crate) use sol::SolInfoEnumCmd;
pub use sol::{
    SolInfo, SolInfoCmd, SolInfoCmdBr, SolInfoEnumCmdBr, SolInfoEnumError, SolInfoFull, SolInfoMode, SolInfoPartial,
};
pub use src::{
    SrcInfo, SrcInfoFull, SrcInfoMode, SrcInfoPartial, SrcItemInfo, SrcItemInfos, SrcMutatedItemBaseInfo,
    SrcMutatedItemInfo, SrcMutatedItemInfos,
};

mod fit;
mod fleet;
mod item;
mod refine;
mod sol;
mod src;
