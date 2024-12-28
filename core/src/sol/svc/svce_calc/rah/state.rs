use crate::{
    defs::{AttrVal, SolItemId},
    sol::{svc::svce_calc::SolAttrVal, SolDmgTypes},
    util::sig_round,
};

use super::shared::SIG_DIGITS;

pub(super) struct SolRahState {
    item_id: SolItemId,
    cycling_time: AttrVal,
    resonances: SolDmgTypes<SolAttrVal>,
    resonances_rounded: SolDmgTypes<AttrVal>,
}
impl SolRahState {
    pub(super) fn new(item_id: SolItemId, cycling_time: AttrVal, resonances: SolDmgTypes<SolAttrVal>) -> Self {
        let resonances_rounded = SolDmgTypes::new(
            sig_round(resonances.em.dogma, SIG_DIGITS),
            sig_round(resonances.thermal.dogma, SIG_DIGITS),
            sig_round(resonances.kinetic.dogma, SIG_DIGITS),
            sig_round(resonances.explosive.dogma, SIG_DIGITS),
        );
        Self {
            item_id,
            cycling_time,
            resonances,
            resonances_rounded,
        }
    }
}
