use crate::{misc::DmgKinds, num::PValue, svc::calc::CalcAttrVals, ud::UItemId};

// Initial values of a RAH, non-rounded
#[derive(Copy, Clone)]
pub(super) struct ItemInfo {
    pub(super) uid: UItemId,
    pub(super) resos: DmgKinds<CalcAttrVals>,
    pub(super) cycle_duration: PValue,
    pub(super) shift_amount: PValue,
}
impl ItemInfo {
    pub(super) fn new(
        uid: UItemId,
        res_em: CalcAttrVals,
        res_therm: CalcAttrVals,
        res_kin: CalcAttrVals,
        res_expl: CalcAttrVals,
        cycle_duration: PValue,
        shift_amount: PValue,
    ) -> Self {
        Self {
            uid,
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
