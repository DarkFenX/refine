use crate::{def::AttrVal, misc::DmgKinds, svc::calc::CalcAttrVal};

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
