pub(crate) use ordered_float::OrderedFloat as OF;

/// Full version of the library as a string.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub(crate) const SERVER_TICK_HZ: u8 = 1;
pub(crate) const SERVER_TICK_S: AttrVal = OF(1.0 / SERVER_TICK_HZ as f64);

// Generic type aliases
pub type Id = i32;
pub type Count = u32;
pub type Value = OF<f64>;
// Entity IDs
pub type AbilId = i32;
pub type EveAttrId = i32;
pub type CustomAttrId = i32;
pub type ItemId = u32;
pub type ItemTypeId = i32;
pub type ItemGrpId = i32;
pub type FitId = u32;
pub type FleetId = u32;
pub type DogmaEffectId = i32;
pub type CustomEffectId = i32;
// Misc
pub type AttrVal = OF<f64>;
pub type Idx = usize;
pub type SlotIndex = i32;
