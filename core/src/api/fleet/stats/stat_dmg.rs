use crate::{
    api::{FleetMut, FleetStatAppliedError},
    misc::Spool,
    svc::vast::{StatDmg, StatDmgApplied, StatDmgItemKinds},
    ud::ItemId,
};

impl<'a> FleetMut<'a> {
    pub fn get_stat_dps(&mut self, item_kinds: StatDmgItemKinds, reload: bool, spool: Option<Spool>) -> StatDmg {
        let u_fleet = self.sol.u_data.fleets.get(self.uid);
        self.sol
            .svc
            .get_stat_fits_dps_raw(&self.sol.u_data, u_fleet.iter_fits(), item_kinds, reload, spool)
    }
    pub fn get_stat_dps_applied(
        &mut self,
        item_kinds: StatDmgItemKinds,
        reload: bool,
        spool: Option<Spool>,
        projectee_item_id: &ItemId,
    ) -> Result<StatDmgApplied, FleetStatAppliedError> {
        let projectee_uid = self.get_stat_applied_projectee_uid(projectee_item_id)?;
        let u_fleet = self.sol.u_data.fleets.get(self.uid);
        Ok(self.sol.svc.get_stat_fits_dps_applied(
            &self.sol.u_data,
            u_fleet.iter_fits(),
            item_kinds,
            reload,
            spool,
            projectee_uid,
        ))
    }
    pub fn get_stat_volley(&mut self, item_kinds: StatDmgItemKinds, spool: Option<Spool>) -> StatDmg {
        let u_fleet = self.sol.u_data.fleets.get(self.uid);
        self.sol
            .svc
            .get_stat_fits_volley_raw(&self.sol.u_data, u_fleet.iter_fits(), item_kinds, spool)
    }
    pub fn get_stat_volley_applied(
        &mut self,
        item_kinds: StatDmgItemKinds,
        spool: Option<Spool>,
        projectee_item_id: &ItemId,
    ) -> Result<StatDmgApplied, FleetStatAppliedError> {
        let projectee_uid = self.get_stat_applied_projectee_uid(projectee_item_id)?;
        let u_fleet = self.sol.u_data.fleets.get(self.uid);
        Ok(self.sol.svc.get_stat_fits_volley_applied(
            &self.sol.u_data,
            u_fleet.iter_fits(),
            item_kinds,
            spool,
            projectee_uid,
        ))
    }
}
