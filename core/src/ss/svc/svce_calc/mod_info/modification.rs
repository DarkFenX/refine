use crate::defs::AttrVal;

use super::{op::ModOpInfo, src::ModSrcInfo};

pub struct ModInfo {
    pub val: AttrVal,
    pub op: ModOpInfo,
    pub penalized: bool,
    pub src: Vec<ModSrcInfo>,
}
impl ModInfo {
    pub(in crate::ss::svc::svce_calc) fn new(
        val: AttrVal,
        op: ModOpInfo,
        penalized: bool,
        src: Vec<ModSrcInfo>,
    ) -> Self {
        Self {
            val,
            op,
            penalized,
            src,
        }
    }
}
