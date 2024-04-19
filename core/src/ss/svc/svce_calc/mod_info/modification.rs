use crate::{
    defs::AttrVal,
    ss::svc::svce_calc::{SsModOpInfo, SsModSrcInfo},
};

pub struct SsModInfo {
    pub val: AttrVal,
    pub op: SsModOpInfo,
    pub penalized: bool,
    pub src: Vec<SsModSrcInfo>,
}
impl SsModInfo {
    pub(in crate::ss::svc::svce_calc) fn new(
        val: AttrVal,
        op: SsModOpInfo,
        penalized: bool,
        src: Vec<SsModSrcInfo>,
    ) -> Self {
        Self {
            val,
            op,
            penalized,
            src,
        }
    }
}
