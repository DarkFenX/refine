pub(in crate::api::item::shared::mutation) use abs_resolve::resolve_absolutes_into_rolls_with_ids;
pub use add::MutationAddError;
pub use full_attr::{FullMAttr, FullMAttrIter, FullMAttrMut};
pub use mutation::{
    DormantMutation, DormantMutationMut, EffectiveMutation, EffectiveMutationMut, Mutation, MutationMut,
};
pub use raw_attr::{AttrMutateRawError, RawMAttr, RawMAttrGetError, RawMAttrIter, RawMAttrMut};

mod abs_resolve;
mod add;
mod change_attrs;
mod full_attr;
mod get;
mod get_base_type_id;
mod get_mutator_type_id;
mod mutation;
mod raw_attr;
mod remove;
mod set_mutator_type_id;
