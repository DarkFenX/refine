// Generic aliases
pub type Idx = usize;
pub type Amount = u32;
// EVE-specific aliases, but not specific to any entity category
pub type AggrKey = i32;
pub type AttrVal = f64;
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
pub type EMutaId = i32;
// Aliases for solar system-specific entities
pub type SsItemId = u32;
pub type SsFitId = u32;
pub type SsFleetId = u32;

/// Full version of the library as a string.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
