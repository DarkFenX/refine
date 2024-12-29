use ordered_float::OrderedFloat as OF;

use crate::{
    defs::AttrVal,
    sol::{svc::svce_calc::SolAttrVal, SolDmgTypes},
    util::sig_round,
};

use super::shared::SIG_DIGITS;

#[derive(Copy, Clone)]
pub(super) struct SolRahSimRahData {
    // Info about initial RAH attributes
    pub(super) resos: SolDmgTypes<SolAttrVal>,
    pub(super) cycle_time: AttrVal,
    pub(super) cycle_time_rounded: AttrVal,
    pub(super) shift_amount: AttrVal,
    // RAH sim state
    pub(super) taken_dmg: SolDmgTypes<AttrVal>,
}
impl SolRahSimRahData {
    pub(super) fn new(
        em: SolAttrVal,
        therm: SolAttrVal,
        kin: SolAttrVal,
        expl: SolAttrVal,
        cycle_time: AttrVal,
        shift_amount: AttrVal,
    ) -> Self {
        Self {
            resos: SolDmgTypes::new(em, therm, kin, expl),
            cycle_time,
            cycle_time_rounded: sig_round(cycle_time, SIG_DIGITS),
            shift_amount,
            taken_dmg: SolDmgTypes::new(OF(0.0), OF(0.0), OF(0.0), OF(0.0)),
        }
    }
}
