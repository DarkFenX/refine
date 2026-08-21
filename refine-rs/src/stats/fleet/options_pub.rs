use crate::{
    CmdResps, ItemId, ItemIdBr,
    err::BrResolveError,
    stats::{
        FleetStats, StatOptionExt, StatOptionFitDmg, StatOptionFitMining, StatOptionFitOutCps, StatOptionFitOutNps,
        StatOptionFitOutRps, StatOptionMass,
        fleet::FleetStatsOptionsInt,
        option::{StatOptionRaw, StatOptionResolved},
    },
};

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(default, bound(deserialize = "I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct FleetStatsOptions<I = ItemId>
{
    #[cfg_attr(feature = "serde", serde(default))]
    default: bool = false,
    #[cfg_attr(feature = "serde", serde(flatten))]
    options: FleetStatsOptionsInt<StatOptionRaw, I>,
}
impl<I> Default for FleetStatsOptions<I> {
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
impl<I> FleetStatsOptions<I> {
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
    pub(crate) fn execute(self, core_fleet: &mut rc::FleetMut) -> FleetStats {
        self.options.stat_resolve(self.default).execute(core_fleet)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetStatsOptionsBr {
    pub(super) fn br_resolve(self, resps: &CmdResps) -> Result<FleetStatsOptions, BrResolveError> {
        Ok(FleetStatsOptions {
            default: self.default,
            options: self.options.br_resolve(resps)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Default + stat resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetStatsOptions {
    pub(super) fn stat_resolve(self) -> FleetStatsOptionsInt<StatOptionResolved, ItemId> {
        self.options.stat_resolve(self.default)
    }
}

impl From<FleetStatsOptions<ItemId>> for FleetStatsOptionsInt<StatOptionResolved, ItemId> {
    fn from(value: FleetStatsOptions<ItemId>) -> Self {
        value.options.stat_resolve(value.default)
    }
}
