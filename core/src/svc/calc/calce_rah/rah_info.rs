use crate::{def::AttrVal, misc::DmgKinds, svc::calc::CalcAttrVals};

// Initial values of a RAH, non-rounded
#[derive(Copy, Clone)]
pub(super) struct RahInfo {
    pub(super) resos: DmgKinds<CalcAttrVals>,
    pub(super) cycle_time: AttrVal,
    pub(super) shift_amount: AttrVal,
}
impl RahInfo {
    pub(super) fn new(
        res_em: CalcAttrVals,
        res_therm: CalcAttrVals,
        res_kin: CalcAttrVals,
        res_expl: CalcAttrVals,
        cycle_time: AttrVal,
        shift_amount: AttrVal,
    ) -> Self {
        Self {
            resos: DmgKinds {
                em: res_em,
                thermal: res_therm,
                kinetic: res_kin,
                explosive: res_expl,
            },
            cycle_time,
            shift_amount,
        }
    }
}
