use super::shared::SIG_ROUND_DIGITS;
use crate::{
    misc::DmgKinds,
    num::{PValue, Value},
    svc::calc::CalcAttrVals,
    ud::UItemId,
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) struct RahSimHistoryEntry {
    pub(super) item_uid: UItemId,
    pub(super) resonances: DmgKinds<Value>,
    pub(super) cycle_time_rounded: PValue,
}
impl RahSimHistoryEntry {
    pub(super) fn new(
        item_uid: UItemId,
        cycle_time: PValue,
        resonances: &DmgKinds<CalcAttrVals>,
        round_resos: bool,
    ) -> Self {
        let resonances = match round_resos {
            true => DmgKinds {
                em: resonances.em.dogma.sig_rounded(SIG_ROUND_DIGITS),
                thermal: resonances.thermal.dogma.sig_rounded(SIG_ROUND_DIGITS),
                kinetic: resonances.kinetic.dogma.sig_rounded(SIG_ROUND_DIGITS),
                explosive: resonances.explosive.dogma.sig_rounded(SIG_ROUND_DIGITS),
            },
            false => DmgKinds {
                em: resonances.em.dogma.into(),
                thermal: resonances.thermal.dogma.into(),
                kinetic: resonances.kinetic.dogma.into(),
                explosive: resonances.explosive.dogma.into(),
            },
        };
        Self {
            item_uid,
            resonances,
            cycle_time_rounded: cycle_time.sig_rounded(SIG_ROUND_DIGITS),
        }
    }
}
