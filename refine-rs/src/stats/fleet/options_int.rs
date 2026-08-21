#[cfg(feature = "serde")]
use crate::stats::option::DeStatOptionKind;
use crate::{
    CmdResps, ItemId, ItemIdBr,
    err::BrResolveError,
    stats::{
        StatOptionFitDmg, StatOptionFitMining, StatOptionFitOutCps, StatOptionFitOutNps, StatOptionFitOutRps,
        StatOptionMass,
        option::{StatOptionExtended, StatOptionKind, StatOptionRaw, StatOptionResolved},
    },
};

pub(in crate::stats) type FleetStatsOptionsResolved = FleetStatsOptionsInt<StatOptionResolved, ItemId>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(default, bound(deserialize = "O: DeStatOptionKind, I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub(in crate::stats) struct FleetStatsOptionsInt<O, I>
where
    O: StatOptionKind,
{
    pub(super) dmg: StatOptionExtended<O, StatOptionFitDmg<I>>,
    pub(super) mps: StatOptionExtended<O, StatOptionFitMining>,
    pub(super) outgoing_nps: StatOptionExtended<O, StatOptionFitOutNps<I>>,
    pub(super) outgoing_rps: StatOptionExtended<O, StatOptionFitOutRps<I>>,
    pub(super) outgoing_cps: StatOptionExtended<O, StatOptionFitOutCps<I>>,
    pub(super) mass: StatOptionExtended<O, StatOptionMass>,
}
impl<O, I> Default for FleetStatsOptionsInt<O, I>
where
    O: StatOptionKind,
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
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetStatsOptionsInt<StatOptionRaw, ItemIdBr> {
    pub(super) fn br_resolve(
        self,
        resps: &CmdResps,
    ) -> Result<FleetStatsOptionsInt<StatOptionRaw, ItemId>, BrResolveError> {
        Ok(FleetStatsOptionsInt {
            dmg: self.dmg.br_resolve(resps)?,
            mps: self.mps,
            outgoing_nps: self.outgoing_nps.br_resolve(resps)?,
            outgoing_rps: self.outgoing_rps.br_resolve(resps)?,
            outgoing_cps: self.outgoing_cps.br_resolve(resps)?,
            mass: self.mass,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Default + stat resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl FleetStatsOptionsInt<StatOptionRaw, ItemId> {
    pub(in crate::stats) fn stat_resolve(self, default: bool) -> FleetStatsOptionsResolved {
        FleetStatsOptionsInt {
            dmg: self.dmg.stat_resolve(default),
            mps: self.mps.stat_resolve(default),
            outgoing_nps: self.outgoing_nps.stat_resolve(default),
            outgoing_rps: self.outgoing_rps.stat_resolve(default),
            outgoing_cps: self.outgoing_cps.stat_resolve(default),
            mass: self.mass.stat_resolve(default),
        }
    }
}
