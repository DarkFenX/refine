use crate::stats::{
    StatOptionFitDmg, StatOptionFitMining, StatOptionFitOutCps, StatOptionFitOutNps, StatOptionFitOutRps,
    StatOptionMass,
    option_support::{StatOptionExtKind, StatOptionExtRaw, StatOptionExtResolved},
};

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(
        default,
        bound(deserialize = "
        C::Repr<StatOptionFitDmg>: Default + serde::Deserialize<'de>,
        C::Repr<StatOptionFitMining>: Default + serde::Deserialize<'de>,
        C::Repr<StatOptionFitOutNps>: Default + serde::Deserialize<'de>,
        C::Repr<StatOptionFitOutRps>: Default + serde::Deserialize<'de>,
        C::Repr<StatOptionFitOutCps>: Default + serde::Deserialize<'de>,
        C::Repr<StatOptionMass>: Default + serde::Deserialize<'de>")
    )
)]
#[derive(Clone)]
pub(in crate::stats) struct FleetStatsOptions<X: StatOptionExtKind> {
    pub(in crate::stats) dmg: X::Repr<StatOptionFitDmg>,
    pub(in crate::stats) mps: X::Repr<StatOptionFitMining>,
    pub(in crate::stats) outgoing_nps: X::Repr<StatOptionFitOutNps>,
    pub(in crate::stats) outgoing_rps: X::Repr<StatOptionFitOutRps>,
    pub(in crate::stats) outgoing_cps: X::Repr<StatOptionFitOutCps>,
    pub(in crate::stats) mass: X::Repr<StatOptionMass>,
}
impl<X> Default for FleetStatsOptions<X>
where
    X: StatOptionExtKind,
    X::Repr<StatOptionFitDmg>: Default,
    X::Repr<StatOptionFitMining>: Default,
    X::Repr<StatOptionFitOutNps>: Default,
    X::Repr<StatOptionFitOutRps>: Default,
    X::Repr<StatOptionFitOutCps>: Default,
    X::Repr<StatOptionMass>: Default,
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
impl FleetStatsOptions<StatOptionExtRaw> {
    pub(in crate::stats) fn resolve(self, default: bool) -> FleetStatsOptions<StatOptionExtResolved> {
        FleetStatsOptions {
            dmg: self.dmg.into_enabled(default),
            mps: self.mps.into_enabled(default),
            outgoing_nps: self.outgoing_nps.into_enabled(default),
            outgoing_rps: self.outgoing_rps.into_enabled(default),
            outgoing_cps: self.outgoing_cps.into_enabled(default),
            mass: self.mass.into_enabled(default),
        }
    }
}
