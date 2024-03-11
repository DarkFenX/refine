use crate::{defs::AttrVal, shr::ModAggrMode};

use super::{op::ModOpInfo, src::ModSrcInfo};

pub struct ModInfo {
    pub val: AttrVal,
    pub op: ModOpInfo,
    pub penalized: bool,
    pub aggr_mode: ModAggrMode,
    pub src: Vec<ModSrcInfo>,
}
impl ModInfo {
    pub(in crate::ss::svc::svce_calc) fn new(
        val: AttrVal,
        op: ModOpInfo,
        penalized: bool,
        aggr_mode: ModAggrMode,
        src: Vec<ModSrcInfo>,
    ) -> Self {
        Self {
            val,
            op,
            penalized,
            aggr_mode,
            src,
        }
    }
}
