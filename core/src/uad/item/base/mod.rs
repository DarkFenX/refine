mod base;
mod base_effect_resolver;
mod mutable;

pub(in crate::uad::item) use base::UadItemBase;
pub(crate) use base_effect_resolver::UadEffectUpdates;
pub(in crate::uad::item) use mutable::UadItemBaseMutable;
pub(crate) use mutable::{ItemMutationData, get_combined_a_attr_values};
