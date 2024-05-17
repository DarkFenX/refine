pub(in crate::sol::item) use effect_modes::SolEffectModes;
pub(in crate::sol::item) use func::{bool_to_state, state_to_bool};
pub(in crate::sol::item) use proj::SolProjs;
pub use state::SolItemState;

pub(in crate::sol::item) mod debug;
mod effect_modes;
mod func;
mod proj;
mod state;
