use crate::defs::AttrVal;

#[derive(Copy, Clone)]
pub(super) struct SolRahInfo {
    pub(super) em: AttrVal,
    pub(super) therm: AttrVal,
    pub(super) kin: AttrVal,
    pub(super) expl: AttrVal,
    pub(super) cycle_time: AttrVal,
    pub(super) shift_amount: AttrVal,
}
impl SolRahInfo {
    pub(super) fn new(
        em: AttrVal,
        therm: AttrVal,
        kin: AttrVal,
        expl: AttrVal,
        cycle_time: AttrVal,
        shift_amount: AttrVal,
    ) -> Self {
        Self {
            em,
            therm,
            kin,
            expl,
            cycle_time,
            shift_amount,
        }
    }
}
