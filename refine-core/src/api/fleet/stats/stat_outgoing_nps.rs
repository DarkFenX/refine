use crate::{
    Fleet, ItemId, PValue,
    stats::{StatNeutItemKinds, StatTimeOptions, err::StatFleetAppliedError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Fleet<'s> {
    pub fn get_stat_outgoing_nps(&mut self, item_kinds: StatNeutItemKinds, time_options: StatTimeOptions) -> PValue {
        let u_fleet = self.sol.u_data.fleets.get(self.uid);
        self.sol.reuse.cseq_cache.clear();
        self.sol.svc.get_stat_fits_outgoing_nps(
            &mut self.sol.reuse.cseq_cache,
            &self.sol.u_data,
            u_fleet.iter_fits(),
            item_kinds,
            time_options,
            None,
        )
    }
    pub fn get_stat_outgoing_nps_applied(
        &mut self,
        item_kinds: StatNeutItemKinds,
        time_options: StatTimeOptions,
        projectee_item_id: &ItemId,
    ) -> Result<PValue, StatFleetAppliedError> {
        let projectee_uid = self.get_stat_applied_projectee_uid(projectee_item_id)?;
        let u_fleet = self.sol.u_data.fleets.get(self.uid);
        self.sol.reuse.cseq_cache.clear();
        Ok(self.sol.svc.get_stat_fits_outgoing_nps(
            &mut self.sol.reuse.cseq_cache,
            &self.sol.u_data,
            u_fleet.iter_fits(),
            item_kinds,
            time_options,
            Some(projectee_uid),
        ))
    }
}
