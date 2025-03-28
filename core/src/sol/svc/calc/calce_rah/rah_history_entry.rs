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
            true => DmgKinds {
                em: rah_round(resonances.em.dogma),
                thermal: rah_round(resonances.thermal.dogma),
                kinetic: rah_round(resonances.kinetic.dogma),
                explosive: rah_round(resonances.explosive.dogma),
            },
            false => DmgKinds {
                em: resonances.em.dogma,
                thermal: resonances.thermal.dogma,
                kinetic: resonances.kinetic.dogma,
                explosive: resonances.explosive.dogma,
            },
        };
        Self {
            item_id,
            resonances,
            cycling_time_rounded: rah_round(cycling_time),
        }
    }
}
