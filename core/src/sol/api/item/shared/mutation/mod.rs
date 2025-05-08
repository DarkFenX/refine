pub(in crate::sol::api::item::shared::mutation) use abs_resolve::{
    resolve_absolutes_into_rolls_with_attrs, resolve_absolutes_into_rolls_with_ids,
    resolve_absolutes_into_rolls_with_items,
};
pub use add::AddMutationError;
pub use full_attr::{FullMAttr, FullMAttrIter, FullMAttrMut};
pub use mutation::{
    EffectiveMutation, EffectiveMutationMut, IncompleteMutation, IncompleteMutationMut, Mutation, MutationMut,
};
pub use raw_attr::{AttrMutateRawError, GetRawMAttrError, RawMAttr, RawMAttrIter, RawMAttrMut};

mod abs_resolve;
mod add;
mod change_attrs;
mod full_attr;
mod get;
mod get_base_type_id;
mod get_mutator_id;
mod mutation;
mod raw_attr;
mod remove;
