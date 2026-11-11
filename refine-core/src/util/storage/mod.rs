pub(crate) use dense::{DenseId, PDense, PDenseRm, SDense, SDenseUnchecked};
pub(crate) use hash::{CMap, Map, RMap, RMapRMap, RMapRMapRMap, RMapRSet, RMapVec, ROrdMap, ROrdSet, RSet, Set};
pub(crate) use hybrid::{SDenseRSet, SmallRSet};

mod dense;
mod hash;
mod hybrid;
