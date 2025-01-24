pub use ordered_float::OrderedFloat as OF;

// Generic aliases
pub type Idx = usize;
pub type Count = u32;
// EVE-specific aliases, but not specific to any entity category
pub(crate) type AggrKey = i32;
pub type AttrVal = OF<f64>;
pub type MutaRoll = OF<f64>;
pub type SkillLevel = u8;
pub type SlotNumber = i32;
// Aliases for EVE-related entities
pub type EAbilId = i32;
pub type EAttrId = i32;
pub type EAttrUnitId = i32;
pub type EBuffId = i32;
pub type EEffectId = i32;
pub type EEffectCatId = i32;
pub type EItemId = i32;
pub type EItemGrpId = i32;
pub type EItemCatId = i32;
// Aliases for solar system-specific entities
pub type SolItemId = u32;
pub type SolFitId = u32;
pub type SolFleetId = u32;

/// Full version of the library as a string.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
