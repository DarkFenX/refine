use crate::{
    defs::AttrVal,
    sol::svc::svce_calc::{SolModOpInfo, SolModSrcInfo},
};

pub struct SolModInfo {
    pub val: AttrVal,
    pub op: SolModOpInfo,
    pub penalized: bool,
    pub src: Vec<SolModSrcInfo>,
}
impl SolModInfo {
    pub(in crate::sol::svc::svce_calc) fn new(
        val: AttrVal,
        op: SolModOpInfo,
        penalized: bool,
        src: Vec<SolModSrcInfo>,
    ) -> Self {
        Self {
            val,
            op,
            penalized,
            src,
        }
    }
}
