mod base;
mod mutable;

pub(in crate::uad::item) use base::UadItemBase;
pub(in crate::uad::item) use mutable::UadItemBaseMutable;
pub(crate) use mutable::{ItemMutationData, get_combined_a_attr_values};
