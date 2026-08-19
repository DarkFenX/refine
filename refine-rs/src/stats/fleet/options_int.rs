use crate::stats::{
    StatOptionFitDmg, StatOptionFitMining, StatOptionFitOutCps, StatOptionFitOutNps, StatOptionFitOutRps,
    StatOptionMass,
    option::{StatOptionKind, StatOptionRaw, StatOptionResolved},
};

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(
        default,
        bound(deserialize = "
        O::Ext<StatOptionFitDmg>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionFitMining>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionFitOutNps>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionFitOutRps>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionFitOutCps>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionMass>: Default + serde::Deserialize<'de>")
    )
)]
#[derive(Clone)]
pub(in crate::stats) struct FleetStatsOptionsInt<O: StatOptionKind> {
    pub(in crate::stats) dmg: O::Ext<StatOptionFitDmg>,
    pub(in crate::stats) mps: O::Ext<StatOptionFitMining>,
    pub(in crate::stats) outgoing_nps: O::Ext<StatOptionFitOutNps>,
    pub(in crate::stats) outgoing_rps: O::Ext<StatOptionFitOutRps>,
    pub(in crate::stats) outgoing_cps: O::Ext<StatOptionFitOutCps>,
    pub(in crate::stats) mass: O::Ext<StatOptionMass>,
}
impl<O> Default for FleetStatsOptionsInt<O>
where
    O: StatOptionKind,
    O::Ext<StatOptionFitDmg>: Default,
    O::Ext<StatOptionFitMining>: Default,
    O::Ext<StatOptionFitOutNps>: Default,
    O::Ext<StatOptionFitOutRps>: Default,
    O::Ext<StatOptionFitOutCps>: Default,
    O::Ext<StatOptionMass>: Default,
{
    fn default() -> Self {
        Self {
            dmg: Default::default(),
            mps: Default::default(),
            outgoing_nps: Default::default(),
            outgoing_rps: Default::default(),
            outgoing_cps: Default::default(),
            mass: Default::default(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetStatsOptionsInt<StatOptionRaw> {
    pub(in crate::stats) fn resolve(self, default: bool) -> FleetStatsOptionsInt<StatOptionResolved> {
        FleetStatsOptionsInt {
            dmg: self.dmg.into_enabled(default),
            mps: self.mps.into_enabled(default),
            outgoing_nps: self.outgoing_nps.into_enabled(default),
            outgoing_rps: self.outgoing_rps.into_enabled(default),
            outgoing_cps: self.outgoing_cps.into_enabled(default),
            mass: self.mass.into_enabled(default),
        }
    }
}
