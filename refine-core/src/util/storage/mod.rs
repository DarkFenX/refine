pub(crate) use dense_id::DenseId;
pub(crate) use hash::{CMap, Map, RDedupMap, RMap, RMapRMap, RMapRMapRMap, RMapRSet, RMapVec, ROrdSet, RSet, Set};
pub(crate) use hybrid::{SSLabRSet, SmallRSet};
pub(crate) use slab::{PSlab, SSlab, SSlabUnchecked};

mod dense_id;
mod hash;
mod hybrid;
mod slab;
