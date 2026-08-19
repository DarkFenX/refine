use crate::stats::{
    StatFleetResult, StatOptionExt, StatOptionFitDmg, StatOptionFitMining, StatOptionFitOutCps, StatOptionFitOutNps,
    StatOptionFitOutRps, StatOptionMass, fleet::StatFleetOptionsInt, option::StatOptionRaw,
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(default))]
#[derive(Default)]
pub struct StatFleetOptions {
    #[cfg_attr(feature = "serde", serde(default = "custom_serde::stat_default"))]
    default: bool = true,
    #[cfg_attr(feature = "serde", serde(flatten))]
    options: StatFleetOptionsInt<StatOptionRaw>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatFleetOptions {
    /// True to have all supported stats enabled by default, false to have them disabled.
    pub fn new(default: bool) -> Self {
        Self {
            default,
            options: StatFleetOptionsInt::default(),
        }
    }
    pub fn with_dmg(mut self, option: StatOptionExt<StatOptionFitDmg>) -> Self {
        self.options.dmg = option.into();
        self
    }
    pub fn with_mps(mut self, option: StatOptionExt<StatOptionFitMining>) -> Self {
        self.options.mps = option.into();
        self
    }
    pub fn with_outgoing_nps(mut self, option: StatOptionExt<StatOptionFitOutNps>) -> Self {
        self.options.outgoing_nps = option.into();
        self
    }
    pub fn with_outgoing_rps(mut self, option: StatOptionExt<StatOptionFitOutRps>) -> Self {
        self.options.outgoing_rps = option.into();
        self
    }
    pub fn with_outgoing_cps(mut self, option: StatOptionExt<StatOptionFitOutCps>) -> Self {
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
impl StatFleetOptions {
    pub(crate) fn execute(self, core_fleet: &mut rc::FleetMut) -> StatFleetResult {
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
