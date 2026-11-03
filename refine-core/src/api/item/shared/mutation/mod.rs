pub(in crate::api::item::shared::mutation) use abs_resolve::resolve_absolute_into_roll;
pub use add::MutationAddError;
pub use full_attr::{FullMAttr, FullMAttrGetError, FullMAttrIter, FullMAttrValueSetError};
pub use mutation::{DormantMutation, EffectiveMutation, Mutation};
pub use raw_attr::{AttrMutateRawError, RawMAttr, RawMAttrGetError, RawMAttrIter};

mod abs_resolve;
mod add;
mod change_attr;
mod full_attr;
mod get;
mod get_base_type_id;
mod get_mutator_type_id;
mod mutation;
mod raw_attr;
mod remove;
mod set_mutator_type_id;
