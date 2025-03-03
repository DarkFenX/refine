use crate::{
    defs::{AttrVal, SolItemId},
    sol::{SolDmgKinds, svc::calc::SolAttrVal},
};

use super::shared::rah_round;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct SolRahSimHistoryEntry {
    pub(super) item_id: SolItemId,
    pub(super) resonances: SolDmgKinds<AttrVal>,
    pub(super) cycling_time_rounded: AttrVal,
}
impl SolRahSimHistoryEntry {
    pub(super) fn new(
        item_id: SolItemId,
        cycling_time: AttrVal,
        resonances: &SolDmgKinds<SolAttrVal>,
        round_resos: bool,
    ) -> Self {
        let resonances = match round_resos {
            true => SolDmgKinds::new(
                rah_round(resonances.em.dogma),
                rah_round(resonances.thermal.dogma),
                rah_round(resonances.kinetic.dogma),
                rah_round(resonances.explosive.dogma),
            ),
            false => SolDmgKinds::new(
                resonances.em.dogma,
                resonances.thermal.dogma,
                resonances.kinetic.dogma,
                resonances.explosive.dogma,
            ),
        };
        Self {
            item_id,
            resonances,
            cycling_time_rounded: rah_round(cycling_time),
        }
    }
}
