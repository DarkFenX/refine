use crate::{
    api::{FleetMut, FleetStatAppliedError},
    def::{AttrVal, ItemId},
    svc::vast::{StatNeutItemKinds, StatTimeOptions},
};

impl<'a> FleetMut<'a> {
    pub fn get_stat_outgoing_nps(&mut self, item_kinds: StatNeutItemKinds, time_options: StatTimeOptions) -> AttrVal {
        let u_fleet = self.sol.u_data.fleets.get(self.key);
        self.sol
            .svc
            .get_stat_fits_outgoing_nps(&self.sol.u_data, u_fleet.iter_fits(), item_kinds, time_options, None)
    }
    pub fn get_stat_outgoing_nps_applied(
        &mut self,
        item_kinds: StatNeutItemKinds,
        time_options: StatTimeOptions,
        projectee_item_id: &ItemId,
    ) -> Result<AttrVal, FleetStatAppliedError> {
        let projectee_key = self.get_stat_applied_projectee_key(projectee_item_id)?;
        let u_fleet = self.sol.u_data.fleets.get(self.key);
        Ok(self.sol.svc.get_stat_fits_outgoing_nps(
            &self.sol.u_data,
            u_fleet.iter_fits(),
            item_kinds,
            time_options,
            Some(projectee_key),
        ))
    }
}
