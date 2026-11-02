pub(crate) use err::{ItemMutatedError, MutatorInplaceError};
pub(in crate::ud::item) use main::UItemBaseMutable;
pub(crate) use main::{ItemMutationData, get_combined_attr_values};
pub(crate) use request::{UAttrMutationRequest, UItemMutationRequest};

mod err;
mod main;
mod request;
