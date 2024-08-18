//! Solar system extension methods which handle calculation-related methods.

pub use sole_get_item_attr::GetItemAttrError;
pub use sole_iter_item_attrs::IterItemAttrsError;
pub use sole_iter_item_effects::IterItemEffectsError;
pub use sole_iter_item_modifiers::IterItemModifiersError;
pub use sole_set_item_effect_mode::SetItemEffectModeError;
pub use sole_set_item_effect_modes::SetItemEffectModesError;

mod sole_get_item_attr;
mod sole_iter_item_attrs;
mod sole_iter_item_effects;
mod sole_iter_item_modifiers;
mod sole_set_item_effect_mode;
mod sole_set_item_effect_modes;
