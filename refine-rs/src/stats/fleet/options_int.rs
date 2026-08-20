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
        O::Ext<StatOptionFitDmg<I>>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionFitMining>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionFitOutNps<I>>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionFitOutRps<I>>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionFitOutCps<I>>: Default + serde::Deserialize<'de>,
        O::Ext<StatOptionMass>: Default + serde::Deserialize<'de>")
    )
)]
#[derive(Clone)]
pub(in crate::stats) struct FleetStatsOptionsInt<O, I>
where
    O: StatOptionKind,
    I: Clone,
{
    pub(in crate::stats) dmg: O::Ext<StatOptionFitDmg<I>>,
    pub(in crate::stats) mps: O::Ext<StatOptionFitMining>,
    pub(in crate::stats) outgoing_nps: O::Ext<StatOptionFitOutNps<I>>,
    pub(in crate::stats) outgoing_rps: O::Ext<StatOptionFitOutRps<I>>,
    pub(in crate::stats) outgoing_cps: O::Ext<StatOptionFitOutCps<I>>,
    pub(in crate::stats) mass: O::Ext<StatOptionMass>,
}
impl<O, I> Default for FleetStatsOptionsInt<O, I>
where
    O: StatOptionKind,
    I: Clone,
    O::Ext<StatOptionFitDmg<I>>: Default,
    O::Ext<StatOptionFitMining>: Default,
    O::Ext<StatOptionFitOutNps<I>>: Default,
    O::Ext<StatOptionFitOutRps<I>>: Default,
    O::Ext<StatOptionFitOutCps<I>>: Default,
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
impl<I> FleetStatsOptionsInt<StatOptionRaw, I>
where
    I: Clone,
{
    pub(in crate::stats) fn resolve(self, default: bool) -> FleetStatsOptionsInt<StatOptionResolved, I> {
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
