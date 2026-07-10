use crate::{nd::NEffectMiningAmount, num::PValue};

#[derive(Copy, Clone, Default)]
pub struct StatMining {
    pub ore: StatMiningEntry,
    pub ice: StatMiningEntry,
    pub gas: StatMiningEntry,
}

#[derive(Copy, Clone, Default)]
pub struct StatMiningEntry {
    pub yield_: PValue,
    pub drain: PValue,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatMiningEntry {
    pub(super) fn from_effect_amount(effect_amount: NEffectMiningAmount) -> Self {
        Self {
            yield_: effect_amount.yield_,
            drain: effect_amount.drain,
        }
    }
}
