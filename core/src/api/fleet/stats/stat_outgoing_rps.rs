use crate::{
    api::{FleetMut, FleetStatAppliedError},
    misc::PValue,
    svc::vast::{StatOutRepItemKinds, StatTank, StatTimeOptions},
    ud::ItemId,
};

impl<'a> FleetMut<'a> {
    pub fn get_stat_outgoing_rps(
        &mut self,
        item_kinds: StatOutRepItemKinds,
        time_options: StatTimeOptions,
    ) -> StatTank<PValue> {
        let u_fleet = self.sol.u_data.fleets.get(self.uid);
        self.sol
            .svc
            .get_stat_fits_outgoing_rps(&self.sol.u_data, u_fleet.iter_fits(), item_kinds, time_options, None)
    }
    pub fn get_stat_outgoing_rps_applied(
        &mut self,
        item_kinds: StatOutRepItemKinds,
        time_options: StatTimeOptions,
        projectee_item_id: &ItemId,
    ) -> Result<StatTank<PValue>, FleetStatAppliedError> {
        let projectee_uid = self.get_stat_applied_projectee_uid(projectee_item_id)?;
        let u_fleet = self.sol.u_data.fleets.get(self.uid);
        Ok(self.sol.svc.get_stat_fits_outgoing_rps(
            &self.sol.u_data,
            u_fleet.iter_fits(),
            item_kinds,
            time_options,
            Some(projectee_uid),
        ))
    }
    pub fn get_stat_outgoing_cps(&mut self, time_options: StatTimeOptions) -> PValue {
        let u_fleet = self.sol.u_data.fleets.get(self.uid);
        self.sol
            .svc
            .get_stat_fits_outgoing_cps(&self.sol.u_data, u_fleet.iter_fits(), time_options, None)
    }
    pub fn get_stat_outgoing_cps_applied(
        &mut self,
        time_options: StatTimeOptions,
        projectee_item_id: &ItemId,
    ) -> Result<PValue, FleetStatAppliedError> {
        let projectee_uid = self.get_stat_applied_projectee_uid(projectee_item_id)?;
        let u_fleet = self.sol.u_data.fleets.get(self.uid);
        Ok(self.sol.svc.get_stat_fits_outgoing_cps(
            &self.sol.u_data,
            u_fleet.iter_fits(),
            time_options,
            Some(projectee_uid),
        ))
    }
}
