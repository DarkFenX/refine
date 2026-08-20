use crate::{
    ItemId, ItemIdBr,
    stats::{
        FleetStatsResult, StatOptionExt, StatOptionFitDmg, StatOptionFitMining, StatOptionFitOutCps,
        StatOptionFitOutNps, StatOptionFitOutRps, StatOptionMass, fleet::FleetStatsOptionsInt, option::StatOptionRaw,
    },
};

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(
        default,
        bound(deserialize = "FleetStatsOptionsInt<StatOptionRaw, I>: Default + serde::Deserialize<'de>")
    )
)]
#[derive(Clone)]
pub struct FleetStatsOptions<I = ItemId>
where
    I: Clone,
{
    #[cfg_attr(feature = "serde", serde(default = "custom_serde::stat_default"))]
    default: bool = true,
    #[cfg_attr(feature = "serde", serde(flatten))]
    options: FleetStatsOptionsInt<StatOptionRaw, I>,
}
impl<I> Default for FleetStatsOptions<I>
where
    I: Clone,
    FleetStatsOptionsInt<StatOptionRaw, I>: Default,
{
    fn default() -> Self {
        Self {
            options: Default::default(),
            ..
        }
    }
}

pub type FleetStatsOptionsBr = FleetStatsOptions<ItemIdBr>;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I> FleetStatsOptions<I>
where
    I: Clone,
{
    /// True to have all supported stats enabled by default, false to have them disabled.
    pub fn new(default: bool) -> Self {
        Self {
            default,
            options: FleetStatsOptionsInt::default(),
        }
    }
    pub fn with_dmg(mut self, option: StatOptionExt<StatOptionFitDmg<I>>) -> Self {
        self.options.dmg = option.into();
        self
    }
    pub fn with_mps(mut self, option: StatOptionExt<StatOptionFitMining>) -> Self {
        self.options.mps = option.into();
        self
    }
    pub fn with_outgoing_nps(mut self, option: StatOptionExt<StatOptionFitOutNps<I>>) -> Self {
        self.options.outgoing_nps = option.into();
        self
    }
    pub fn with_outgoing_rps(mut self, option: StatOptionExt<StatOptionFitOutRps<I>>) -> Self {
        self.options.outgoing_rps = option.into();
        self
    }
    pub fn with_outgoing_cps(mut self, option: StatOptionExt<StatOptionFitOutCps<I>>) -> Self {
        self.options.outgoing_cps = option.into();
        self
    }
    pub fn with_mass(mut self, option: StatOptionExt<StatOptionMass>) -> Self {
        self.options.mass = option.into();
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetStatsOptions {
    pub(crate) fn execute(self, core_fleet: &mut rc::FleetMut) -> FleetStatsResult {
        self.options.resolve(self.default).execute(core_fleet)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    pub(super) fn stat_default() -> bool {
        true
    }
}
