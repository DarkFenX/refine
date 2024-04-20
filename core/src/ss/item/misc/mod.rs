pub(in crate::ss::item) use effect_modes::SsEffectModes;
pub(in crate::ss::item) use func::{bool_to_state, state_to_bool};
pub use state::SsItemState;
pub(in crate::ss::item) use tgt_items::SsTgtItems;

pub(in crate::ss::item) mod debug;
mod effect_modes;
mod func;
mod state;
mod tgt_items;
