pub use sol_get_sw_effect::GetSwEffectError;
pub use sw_effect::{SwEffect, SwEffectMut};

mod int_load_unload;
mod sol_add_sw_effect;
mod sol_get_sw_effect;
mod sol_iter_sw_effects;
mod sw_effect;
mod sw_effect_remove;
mod sw_effect_set_state;
mod sw_effect_set_type_id;
