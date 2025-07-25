mod base;
mod base_effect_resolver;
mod mutable;

pub(in crate::uad::item) use base::UadItemBase;
pub(crate) use base_effect_resolver::UadEffectUpdates;
use base_effect_resolver::process_effects;
pub(in crate::uad::item) use mutable::UadItemBaseMutable;
pub(crate) use mutable::{ItemMutationData, get_combined_attr_values};
