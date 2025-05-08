mod base;
mod mutable;

pub(in crate::sol::uad::item) use base::UadItemBase;
pub(in crate::sol::uad::item) use mutable::UadItemBaseMutable;
pub(in crate::sol) use mutable::{ItemMutationData, get_combined_a_attr_values};
