// Generic aliases
pub type ReeInt = i32;
pub type ReeFloat = f64;
pub type ReeId = u32;
pub type ReeIdx = usize;
// Entity-specific aliases
pub type AttrId = i32;
pub type EffectId = i32;
pub type SsItemId = u32;
pub type SsFitId = u32;

pub const REEINT_MAX: ReeInt = ReeInt::MAX;
pub const REEINT_MIN: ReeInt = ReeInt::MIN;
/// Full version of the library as a string.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
