use crate::{
    CmdResps, IdType, ItemId, ItemIdBr,
    err::BrResolveError,
    stats::{
        StatOptionFitDmg, StatOptionFitMining, StatOptionFitOutCps, StatOptionFitOutNps, StatOptionFitOutRps,
        StatOptionMass,
        option::{StatOptionKind, StatOptionRaw, StatOptionResolved},
    },
};

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(default, bound(deserialize = ""))
)]
#[derive(Clone)]
pub(in crate::stats) struct FleetStatsOptionsInt<O, I>
where
    O: StatOptionKind,
    I: IdType,
{
    pub(super) dmg: O::Ext<StatOptionFitDmg<I>>,
    pub(super) mps: O::Ext<StatOptionFitMining>,
    pub(super) outgoing_nps: O::Ext<StatOptionFitOutNps<I>>,
    pub(super) outgoing_rps: O::Ext<StatOptionFitOutRps<I>>,
    pub(super) outgoing_cps: O::Ext<StatOptionFitOutCps<I>>,
    pub(super) mass: O::Ext<StatOptionMass>,
}
impl<O, I> Default for FleetStatsOptionsInt<O, I>
where
    O: StatOptionKind,
    I: IdType,
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
impl<I> FleetStatsOptionsInt<StatOptionRaw, I>
where
    I: IdType,
{
    pub(in crate::stats) fn stat_resolve(self, default: bool) -> FleetStatsOptionsInt<StatOptionResolved, I> {
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
