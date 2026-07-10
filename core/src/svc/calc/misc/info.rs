use smallvec::SmallVec;

use crate::{
    api::Op,
    num::{PValue, Value},
    rd::{RAttrId, RState},
    ud::UItemId,
};

pub(crate) struct CalcModInfo {
    pub(crate) state: Option<RState>,
    // Public operator serves even internal purposes well
    pub(crate) op: Op,
    pub(crate) initial_str: Value,
    pub(crate) range_mult: Option<PValue>,
    pub(crate) resist_mult: Option<PValue>,
    pub(crate) stacking_mult: Option<PValue>,
    pub(crate) applied_str: Value,
    pub(crate) affectors: SmallVec<[CalcModInfoAffector; 1]>,
}

pub(crate) struct CalcModInfoAffector {
    pub(crate) item_uid: UItemId,
    pub(crate) attr_rid: Option<RAttrId>,
}
