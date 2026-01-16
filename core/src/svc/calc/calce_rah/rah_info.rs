use crate::{misc::DmgKinds, num::PValue, svc::calc::CalcAttrVals};

// Initial values of a RAH, non-rounded
#[derive(Copy, Clone)]
pub(super) struct RahInfo {
    pub(super) resos: DmgKinds<CalcAttrVals>,
    pub(super) cycle_duration: PValue,
    pub(super) shift_amount: PValue,
}
impl RahInfo {
    pub(super) fn new(
        res_em: CalcAttrVals,
        res_therm: CalcAttrVals,
        res_kin: CalcAttrVals,
        res_expl: CalcAttrVals,
        cycle_duration: PValue,
        shift_amount: PValue,
    ) -> Self {
        Self {
            resos: DmgKinds {
                em: res_em,
                thermal: res_therm,
                kinetic: res_kin,
                explosive: res_expl,
            },
            cycle_duration,
            shift_amount,
        }
    }
}
