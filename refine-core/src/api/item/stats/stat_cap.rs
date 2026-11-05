use crate::{
    ItemId, OptionalReload, PValue, UnitInterval, Value,
    stats::{
        ItemStats, StatCapBlcSrcKinds, StatCapSim, StatCapSimStagger, StatTimeOptions,
        err::{StatItemAppliedError, StatItemError},
    },
    svc::vast::{StatCapBlcSrcKindsInt, StatCapSimStaggerInt},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> ItemStats<'s> {
    pub fn get_stat_cap_amount(&mut self) -> Result<PValue, StatItemError<!>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        sol.svc
            .get_stat_item_cap_amount(&sol.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items))
    }
    pub fn get_stat_cap_balance(
        &mut self,
        src_kinds: StatCapBlcSrcKinds,
        time_options: StatTimeOptions,
    ) -> Result<Value, StatItemAppliedError<!>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        let src_kinds = StatCapBlcSrcKindsInt::from_pub(&src_kinds, &sol.u_data)?;
        sol.svc
            .get_stat_item_cap_balance(
                &mut sol.reuse.cseq_cache,
                &sol.u_data,
                item_uid,
                src_kinds,
                time_options,
            )
            .map_err(|e| StatItemAppliedError::from_svc_err(e, &sol.u_data.items))
    }
    pub fn get_stat_cap_sim(
        &mut self,
        cap_perc: UnitInterval,
        optional_reloads: Option<OptionalReload>,
        stagger: &StatCapSimStagger,
        nosf_projectee_item_id: Option<&ItemId>,
    ) -> Result<StatCapSim, StatItemAppliedError<!>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        let nosf_projectee_item_uid = match nosf_projectee_item_id {
            Some(nosf_projectee_item_id) => Some(sol.u_data.get_projectee_uid(nosf_projectee_item_id)?),
            None => None,
        };
        let stagger = StatCapSimStaggerInt::from_pub(sol, stagger);
        sol.svc
            .get_stat_item_cap_sim(
                &mut sol.reuse.cseq_cache,
                &sol.u_data,
                item_uid,
                cap_perc,
                optional_reloads,
                &stagger,
                nosf_projectee_item_uid,
            )
            .map_err(|e| StatItemAppliedError::from_svc_err(e, &sol.u_data.items))
    }
    pub fn get_stat_neut_resist(&mut self) -> Result<UnitInterval, StatItemError<!>> {
        let item_uid = self.uid;
        let sol = &mut *self.sol;
        sol.svc
            .get_stat_item_neut_resist(&sol.u_data, item_uid)
            .map_err(|e| StatItemError::from_svc_err(e, &sol.u_data.items))
    }
}
