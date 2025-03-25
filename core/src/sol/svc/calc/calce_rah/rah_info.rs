use crate::sol::{AttrVal, DmgKinds, svc::calc::CalcAttrVal};

// Initial values of a RAH, non-rounded
#[derive(Copy, Clone)]
pub(super) struct RahInfo {
    pub(super) resos: DmgKinds<CalcAttrVal>,
    pub(super) cycle_time: AttrVal,
    pub(super) shift_amount: AttrVal,
}
impl RahInfo {
    pub(super) fn new(
        res_em: CalcAttrVal,
        res_therm: CalcAttrVal,
        res_kin: CalcAttrVal,
        res_expl: CalcAttrVal,
        cycle_time: AttrVal,
        shift_amount: AttrVal,
    ) -> Self {
        Self {
            resos: DmgKinds::new(res_em, res_therm, res_kin, res_expl),
            cycle_time,
            shift_amount,
        }
    }
}
