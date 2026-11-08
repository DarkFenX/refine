use crate::{
    ItemId, PValue,
    sol::SolCore,
    stats::{
        ItemStats, StatCritOptions, StatDmg, StatDmgApplied, StatItemChargeOptions, StatMining, StatMiningResourceKind,
        StatOutReps, StatTimeOptions, SwitchedItemStats,
        err::{StatItemAppliedError, StatItemError},
    },
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Methods shared by regular and state-switched item stats.
#[expect(private_bounds)]
pub trait ItemStatsCommon: ItemStatsSealed {
    fn get_item_id(&self) -> ItemId {
        self.get_sol().u_data.items.ext_id_by_int_id(self.get_uid())
    }
    fn get_stat_dmg(
        &mut self,
        time_options: StatTimeOptions,
        crit_options: StatCritOptions,
        charge_options: StatItemChargeOptions,
    ) -> Result<StatDmg, StatItemError<!>> {
        let item_uid = self.get_uid();
        let switched = self.is_switched();
        let sol = self.get_sol_mut();
        let cseq_cache = match switched {
            true => &mut sol.reuse.cseq_cache_switched,
            false => &mut sol.reuse.cseq_cache,
        };
        sol.svc
            .get_stat_item_dmg_raw(
                cseq_cache,
                &sol.u_data,
                item_uid,
                time_options,
                crit_options,
                charge_options,
            )
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_dmg_applied(
        &mut self,
        time_options: StatTimeOptions,
        crit_options: StatCritOptions,
        charge_options: StatItemChargeOptions,
        projectee_item_id: &ItemId,
    ) -> Result<StatDmgApplied, StatItemAppliedError<!>> {
        let item_uid = self.get_uid();
        let switched = self.is_switched();
        let sol = self.get_sol_mut();
        let projectee_uid = sol.u_data.get_projectee_uid(*projectee_item_id)?;
        let cseq_cache = match switched {
            true => &mut sol.reuse.cseq_cache_switched,
            false => &mut sol.reuse.cseq_cache,
        };
        sol.svc
            .get_stat_item_dmg_applied(
                cseq_cache,
                &sol.u_data,
                item_uid,
                time_options,
                crit_options,
                charge_options,
                projectee_uid,
            )
            .map_err(|e| StatItemAppliedError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_mps(
        &mut self,
        time_options: StatTimeOptions,
        resource_kind: StatMiningResourceKind,
    ) -> Result<StatMining, StatItemError<!>> {
        let item_uid = self.get_uid();
        let switched = self.is_switched();
        let sol = self.get_sol_mut();
        let cseq_cache = match switched {
            true => &mut sol.reuse.cseq_cache_switched,
            false => &mut sol.reuse.cseq_cache,
        };
        sol.svc
            .get_stat_item_mps(cseq_cache, &sol.u_data, item_uid, time_options, resource_kind)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_outgoing_nps(
        &mut self,
        time_options: StatTimeOptions,
        charge_options: StatItemChargeOptions,
    ) -> Result<PValue, StatItemError<!>> {
        let item_uid = self.get_uid();
        let switched = self.is_switched();
        let sol = self.get_sol_mut();
        let cseq_cache = match switched {
            true => &mut sol.reuse.cseq_cache_switched,
            false => &mut sol.reuse.cseq_cache,
        };
        sol.svc
            .get_stat_item_outgoing_nps(cseq_cache, &sol.u_data, item_uid, time_options, charge_options, None)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_outgoing_nps_applied(
        &mut self,
        time_options: StatTimeOptions,
        charge_options: StatItemChargeOptions,
        projectee_item_id: &ItemId,
    ) -> Result<PValue, StatItemAppliedError<!>> {
        let item_uid = self.get_uid();
        let switched = self.is_switched();
        let sol = self.get_sol_mut();
        let projectee_uid = sol.u_data.get_projectee_uid(*projectee_item_id)?;
        let cseq_cache = match switched {
            true => &mut sol.reuse.cseq_cache_switched,
            false => &mut sol.reuse.cseq_cache,
        };
        sol.svc
            .get_stat_item_outgoing_nps(
                cseq_cache,
                &sol.u_data,
                item_uid,
                time_options,
                charge_options,
                Some(projectee_uid),
            )
            .map_err(|e| StatItemAppliedError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_outgoing_rps(&mut self, time_options: StatTimeOptions) -> Result<StatOutReps, StatItemError<!>> {
        let item_uid = self.get_uid();
        let switched = self.is_switched();
        let sol = self.get_sol_mut();
        let cseq_cache = match switched {
            true => &mut sol.reuse.cseq_cache_switched,
            false => &mut sol.reuse.cseq_cache,
        };
        sol.svc
            .get_stat_item_outgoing_rps(cseq_cache, &sol.u_data, item_uid, time_options, None)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_outgoing_rps_applied(
        &mut self,
        time_options: StatTimeOptions,
        projectee_item_id: &ItemId,
    ) -> Result<StatOutReps, StatItemAppliedError<!>> {
        let item_uid = self.get_uid();
        let switched = self.is_switched();
        let sol = self.get_sol_mut();
        let projectee_uid = sol.u_data.get_projectee_uid(*projectee_item_id)?;
        let cseq_cache = match switched {
            true => &mut sol.reuse.cseq_cache_switched,
            false => &mut sol.reuse.cseq_cache,
        };
        sol.svc
            .get_stat_item_outgoing_rps(cseq_cache, &sol.u_data, item_uid, time_options, Some(projectee_uid))
            .map_err(|e| StatItemAppliedError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_outgoing_cps(&mut self, time_options: StatTimeOptions) -> Result<PValue, StatItemError<!>> {
        let item_uid = self.get_uid();
        let switched = self.is_switched();
        let sol = self.get_sol_mut();
        let cseq_cache = match switched {
            true => &mut sol.reuse.cseq_cache_switched,
            false => &mut sol.reuse.cseq_cache,
        };
        sol.svc
            .get_stat_item_outgoing_cps(cseq_cache, &sol.u_data, item_uid, time_options, None)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items))
    }
    fn get_stat_outgoing_cps_applied(
        &mut self,
        time_options: StatTimeOptions,
        projectee_item_id: &ItemId,
    ) -> Result<PValue, StatItemAppliedError<!>> {
        let item_uid = self.get_uid();
        let switched = self.is_switched();
        let sol = self.get_sol_mut();
        let projectee_uid = sol.u_data.get_projectee_uid(*projectee_item_id)?;
        let cseq_cache = match switched {
            true => &mut sol.reuse.cseq_cache_switched,
            false => &mut sol.reuse.cseq_cache,
        };
        sol.svc
            .get_stat_item_outgoing_cps(cseq_cache, &sol.u_data, item_uid, time_options, Some(projectee_uid))
            .map_err(|e| StatItemAppliedError::from_svc_err(e, &sol.u_data.items))
    }
}
impl ItemStatsCommon for ItemStats<'_> {}
impl ItemStatsCommon for SwitchedItemStats<'_> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::api) trait ItemStatsSealed {
    fn get_uid(&self) -> UItemId;
    fn get_sol(&self) -> &SolCore;
    fn get_sol_mut(&mut self) -> &mut SolCore;
    // Switched item states need their own cycle cache
    fn is_switched(&self) -> bool;
}
impl ItemStatsSealed for ItemStats<'_> {
    fn get_uid(&self) -> UItemId {
        self.uid
    }
    fn get_sol(&self) -> &SolCore {
        self.sol
    }
    fn get_sol_mut(&mut self) -> &mut SolCore {
        self.sol
    }
    fn is_switched(&self) -> bool {
        false
    }
}
impl ItemStatsSealed for SwitchedItemStats<'_> {
    fn get_uid(&self) -> UItemId {
        self.uid
    }
    fn get_sol(&self) -> &SolCore {
        self.sol
    }
    fn get_sol_mut(&mut self) -> &mut SolCore {
        self.sol
    }
    fn is_switched(&self) -> bool {
        self.savepoint.is_some()
    }
}
