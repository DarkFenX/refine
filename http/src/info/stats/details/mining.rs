use serde::Serialize;
use serde_tuple::Serialize_tuple;

#[derive(Serialize)]
pub(crate) struct HStatMining {
    #[serde(skip_serializing_if = "HStatMiningAmount::is_null")]
    ore: HStatMiningAmount,
    #[serde(skip_serializing_if = "HStatMiningAmount::is_null")]
    ice: HStatMiningAmount,
    #[serde(skip_serializing_if = "HStatMiningAmount::is_null")]
    gas: HStatMiningAmount,
}

#[derive(Serialize_tuple)]
struct HStatMiningAmount {
    yield_: f64,
    drain: f64,
}
impl HStatMiningAmount {
    fn is_null(&self) -> bool {
        self.yield_ == 0.0 && self.drain == 0.0
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HStatMining {
    pub(crate) fn from_core(core_stat: rc::stats::StatMining) -> Self {
        Self {
            ore: HStatMiningAmount::from_core(core_stat.ore),
            ice: HStatMiningAmount::from_core(core_stat.ice),
            gas: HStatMiningAmount::from_core(core_stat.gas),
        }
    }
}

impl HStatMiningAmount {
    fn from_core(core_stat: rc::MiningAmount) -> Self {
        Self {
            yield_: core_stat.yield_.into_f64(),
            drain: core_stat.drain.into_f64(),
        }
    }
}
