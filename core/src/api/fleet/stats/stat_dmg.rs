use crate::{
    api::{FleetMut, FleetStatAppliedError},
    svc::{
        cycle::CseqMap,
        vast::{StatDmg, StatDmgApplied, StatDmgItemKinds, StatTimeOptions},
    },
    ud::ItemId,
};

impl<'a> FleetMut<'a> {
    pub fn get_stat_dmg(&mut self, item_kinds: StatDmgItemKinds, time_options: StatTimeOptions) -> StatDmg {
        let u_fleet = self.sol.u_data.fleets.get(self.uid);
        self.sol.svc.get_stat_fits_dmg_raw(
            &mut CseqMap::new(),
            &self.sol.u_data,
            u_fleet.iter_fits(),
            item_kinds,
            time_options,
        )
    }
    pub fn get_stat_dmg_applied(
        &mut self,
        item_kinds: StatDmgItemKinds,
        time_options: StatTimeOptions,
        projectee_item_id: &ItemId,
    ) -> Result<StatDmgApplied, FleetStatAppliedError> {
        let projectee_uid = self.get_stat_applied_projectee_uid(projectee_item_id)?;
        let u_fleet = self.sol.u_data.fleets.get(self.uid);
        Ok(self.sol.svc.get_stat_fits_dmg_applied(
            &mut CseqMap::new(),
            &self.sol.u_data,
            u_fleet.iter_fits(),
            item_kinds,
            time_options,
            projectee_uid,
        ))
    }
}
