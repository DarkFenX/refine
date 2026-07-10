pub(crate) use primary::PSlab;
pub(crate) use secondary::{SSlab, SSlabUnchecked};
pub(crate) use shared::SlabId;

mod primary;
mod secondary;
mod shared;
