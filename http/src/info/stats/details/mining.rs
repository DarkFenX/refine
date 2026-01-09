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
impl From<rc::stats::StatMining> for HStatMining {
    fn from(core_stat: rc::stats::StatMining) -> Self {
        Self {
            ore: core_stat.ore.into(),
            ice: core_stat.ice.into(),
            gas: core_stat.gas.into(),
        }
    }
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
impl From<rc::MiningAmount> for HStatMiningAmount {
    fn from(core_stat: rc::MiningAmount) -> Self {
        Self {
            yield_: core_stat.yield_.into_f64(),
            drain: core_stat.drain.into_f64(),
        }
    }
}
