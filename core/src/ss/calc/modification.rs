use crate::{
    consts::{ModAggrMode, ModOp},
    ReeFloat,
};

pub(in crate::ss::calc) struct Modification {
    op: ModOp,
    val: ReeFloat,
    res_val: ReeFloat,
    aggr_mode: ModAggrMode,
    afor_pen_immune: bool,
}
impl Modification {
    pub(in crate::ss::calc) fn new(
        op: ModOp,
        val: ReeFloat,
        res_val: ReeFloat,
        aggr_mode: ModAggrMode,
        afor_pen_immune: bool,
    ) -> Modification {
        Modification {
            op,
            val,
            res_val,
            aggr_mode,
            afor_pen_immune,
        }
    }
}
