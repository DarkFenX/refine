// Generic aliases
pub type ReeIdx = usize;
// Aliases for global entities
pub type AbilId = i32;
pub type AttrId = i32;
pub type AttrUnitId = i32;
pub type BuffId = i32;
pub type EffectId = i32;
pub type EffectCatId = i32;
pub type ItemId = i32;
pub type ItemGrpId = i32;
pub type ItemCatId = i32;
pub type MutaId = i32;
pub type AttrVal = f64;
pub type Amount = u32;
pub type SkillLevel = u8;
pub type SlotNumber = i32;
// Aliases for solar system-specific entities
pub type SsItemId = u32;
pub type SsFitId = u32;

/// Full version of the library as a string.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
