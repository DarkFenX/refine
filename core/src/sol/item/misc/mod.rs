pub(in crate::sol::item) use effect_modes::SolEffectModes;
pub(in crate::sol::item) use func::{bool_to_state, state_to_bool};
pub use state::SolItemState;
pub(in crate::sol::item) use tgt_items::SolTgtItems;

pub(in crate::sol::item) mod debug;
mod effect_modes;
mod func;
mod state;
mod tgt_items;
