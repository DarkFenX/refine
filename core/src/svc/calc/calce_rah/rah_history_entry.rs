use super::shared::rah_round;
use crate::{def::AttrVal, misc::DmgKinds, svc::calc::CalcAttrVal, ud::UItemKey};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct RahSimHistoryEntry {
    pub(super) item_key: UItemKey,
    pub(super) resonances: DmgKinds<AttrVal>,
    pub(super) cycling_time_rounded: AttrVal,
}
impl RahSimHistoryEntry {
    pub(super) fn new(
        item_key: UItemKey,
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
            item_key,
            resonances,
            cycling_time_rounded: rah_round(cycling_time),
        }
    }
}
