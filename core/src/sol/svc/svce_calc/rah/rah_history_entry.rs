use crate::{
    defs::{AttrVal, SolItemId},
    sol::{svc::svce_calc::SolAttrVal, SolDmgTypes},
    util::sig_round,
};

use super::shared::SIG_DIGITS;

#[derive(Copy, Clone)]
pub(super) struct SolRahSimHistoryEntry {
    pub(super) item_id: SolItemId,
    pub(super) resonances: SolDmgTypes<AttrVal>,
    pub(super) cycling_time_rounded: AttrVal,
    resonances_rounded: SolDmgTypes<AttrVal>,
}
impl SolRahSimHistoryEntry {
    pub(super) fn new(item_id: SolItemId, cycling_time: AttrVal, resonances: &SolDmgTypes<SolAttrVal>) -> Self {
        let resonances = SolDmgTypes::new(
            resonances.em.dogma,
            resonances.thermal.dogma,
            resonances.kinetic.dogma,
            resonances.explosive.dogma,
        );
        let resonances_rounded = SolDmgTypes::new(
            sig_round(resonances.em, SIG_DIGITS),
            sig_round(resonances.thermal, SIG_DIGITS),
            sig_round(resonances.kinetic, SIG_DIGITS),
            sig_round(resonances.explosive, SIG_DIGITS),
        );
        Self {
            item_id,
            resonances,
            cycling_time_rounded: sig_round(cycling_time, SIG_DIGITS),
            resonances_rounded,
        }
    }
}
impl Eq for SolRahSimHistoryEntry {}
impl PartialEq for SolRahSimHistoryEntry {
    fn eq(&self, other: &Self) -> bool {
        self.item_id == other.item_id
            // Use only rounded attributes for more reliable loop detection, to avoid float errors
            && self.cycling_time_rounded == other.cycling_time_rounded
            && self.resonances_rounded == other.resonances_rounded
    }
}
impl std::hash::Hash for SolRahSimHistoryEntry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.item_id.hash(state);
        // Use only rounded attributes for more reliable loop detection, to avoid float errors
        self.cycling_time_rounded.hash(state);
        self.resonances_rounded.hash(state);
    }
}
