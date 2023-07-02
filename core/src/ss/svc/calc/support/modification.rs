use crate::{consts::ModAggrMode, defs::AttrVal, shr::ModOp};

pub(in crate::ss::svc::calc) struct Modification {
    pub(in crate::ss::svc::calc) op: ModOp,
    pub(in crate::ss::svc::calc) val: AttrVal,
    pub(in crate::ss::svc::calc) res_val: AttrVal,
    pub(in crate::ss::svc::calc) aggr_mode: ModAggrMode,
    pub(in crate::ss::svc::calc) afor_pen_immune: bool,
}
impl Modification {
    pub(in crate::ss::svc::calc) fn new(
        op: ModOp,
        val: AttrVal,
        res_val: AttrVal,
        aggr_mode: ModAggrMode,
        afor_pen_immune: bool,
    ) -> Self {
        Self {
            op,
            val,
            res_val,
            aggr_mode,
            afor_pen_immune,
        }
    }
}
