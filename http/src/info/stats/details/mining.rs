#[derive(serde::Serialize)]
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

#[derive(serde_tuple::Serialize_tuple)]
struct HStatMiningAmount {
    yield_: rc::AttrVal,
    waste: rc::AttrVal,
}
impl HStatMiningAmount {
    fn is_null(&self) -> bool {
        self.yield_.into_inner() == 0.0 && self.waste.into_inner() == 0.0
    }
}
impl From<rc::MiningAmount> for HStatMiningAmount {
    fn from(core_stat: rc::MiningAmount) -> Self {
        Self {
            yield_: core_stat.yield_,
            waste: core_stat.waste,
        }
    }
}
