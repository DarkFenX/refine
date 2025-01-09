use crate::{
    defs::{AttrVal, SolItemId},
    sol::{svc::calc::SolAttrVal, SolDmgTypes},
};

use super::shared::rah_round;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct SolRahSimHistoryEntry {
    pub(super) item_id: SolItemId,
    pub(super) resonances: SolDmgTypes<AttrVal>,
    pub(super) cycling_time_rounded: AttrVal,
}
impl SolRahSimHistoryEntry {
    pub(super) fn new(
        item_id: SolItemId,
        cycling_time: AttrVal,
        resonances: &SolDmgTypes<SolAttrVal>,
        round_resos: bool,
    ) -> Self {
        let resonances = match round_resos {
            true => SolDmgTypes::new(
                rah_round(resonances.em.dogma),
                rah_round(resonances.thermal.dogma),
                rah_round(resonances.kinetic.dogma),
                rah_round(resonances.explosive.dogma),
            ),
            false => SolDmgTypes::new(
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
