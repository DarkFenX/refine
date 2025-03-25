use crate::sol::{AttrVal, DmgKinds, ItemId, svc::calc::CalcAttrVal};

use super::shared::rah_round;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct RahSimHistoryEntry {
    pub(super) item_id: ItemId,
    pub(super) resonances: DmgKinds<AttrVal>,
    pub(super) cycling_time_rounded: AttrVal,
}
impl RahSimHistoryEntry {
    pub(super) fn new(
        item_id: ItemId,
        cycling_time: AttrVal,
        resonances: &DmgKinds<CalcAttrVal>,
        round_resos: bool,
    ) -> Self {
        let resonances = match round_resos {
            true => DmgKinds::new(
                rah_round(resonances.em.dogma),
                rah_round(resonances.thermal.dogma),
                rah_round(resonances.kinetic.dogma),
                rah_round(resonances.explosive.dogma),
            ),
            false => DmgKinds::new(
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
