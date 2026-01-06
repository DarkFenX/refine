pub(in crate::ud::item) use base::UItemBase;
pub(crate) use base_effect_resolver::UEffectUpdates;
use base_effect_resolver::process_effects;
pub(in crate::ud::item) use mutable::UItemBaseMutable;
pub(crate) use mutable::{
    ItemMutatedError, ItemMutationData, UAttrMutationRequest, UItemMutationRequest, get_combined_attr_values,
};

mod base;
mod base_effect_resolver;
mod mutable;
