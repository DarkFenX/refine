use ordered_float::OrderedFloat as OF;

// Entity IDs
pub type AttrId = i32;
pub type ItemId = u32;
pub type ItemTypeId = i32;
pub type ItemGrpId = i32;
pub type FitId = u32;
pub type FleetId = u32;
pub type DogmaEffectId = i32;
pub type CustomEffectId = i32;
// Misc
pub type AttrVal = OF<f64>;
pub type Count = u32;
pub type Idx = usize;
pub type MutaRoll = OF<f64>;
pub type SecStatus = OF<f64>;
pub type SkillLevel = u8;
pub type SlotIndex = i32;
// Internal-only entity IDs
pub(in crate::sol) type ItemKey = usize;
pub(in crate::sol) type FitKey = usize;
pub(in crate::sol) type FleetKey = usize;
