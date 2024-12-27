use crate::defs::AttrVal;

pub(super) struct SolRahInfo {
    em: AttrVal,
    therm: AttrVal,
    kin: AttrVal,
    expl: AttrVal,
    cycle_time: AttrVal,
    shift_amount: AttrVal,
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
