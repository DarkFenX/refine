use serde_tuple::Serialize_tuple;

#[derive(Serialize_tuple)]
pub(crate) struct HStatMining {
    ore: HStatMiningEntry,
    ice: HStatMiningEntry,
    gas: HStatMiningEntry,
}

#[derive(Serialize_tuple)]
struct HStatMiningEntry {
    yield_: f64,
    drain: f64,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HStatMining {
    pub(crate) fn from_core(core_stat: rc::stats::StatMining) -> Self {
        Self {
            ore: HStatMiningEntry::from_core(core_stat.ore),
            ice: HStatMiningEntry::from_core(core_stat.ice),
            gas: HStatMiningEntry::from_core(core_stat.gas),
        }
    }
}

impl HStatMiningEntry {
    fn from_core(core_stat: rc::stats::StatMiningEntry) -> Self {
        Self {
            yield_: core_stat.yield_.into_f64(),
            drain: core_stat.drain.into_f64(),
        }
    }
}
