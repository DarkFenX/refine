use crate::{
    Fleet, ItemId,
    stats::{StatCritOptions, StatDmg, StatDmgApplied, StatDmgItemKinds, StatTimeOptions, err::StatFleetAppliedError},
    svc::cycle::CseqMap,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Fleet<'s> {
    pub fn get_stat_dmg(
        &mut self,
        item_kinds: StatDmgItemKinds,
        time_options: StatTimeOptions,
        crit_options: StatCritOptions,
    ) -> StatDmg {
        let u_fleet = self.core.u_data.fleets.get(self.uid);
        self.core.svc.get_stat_fits_dmg_raw(
            &mut CseqMap::new(),
            &self.core.u_data,
            u_fleet.iter_fits(),
            item_kinds,
            time_options,
            crit_options,
        )
    }
    pub fn get_stat_dmg_applied(
        &mut self,
        item_kinds: StatDmgItemKinds,
        time_options: StatTimeOptions,
        crit_options: StatCritOptions,
        projectee_item_id: &ItemId,
    ) -> Result<StatDmgApplied, StatFleetAppliedError> {
        let projectee_uid = self.get_stat_applied_projectee_uid(projectee_item_id)?;
        let u_fleet = self.core.u_data.fleets.get(self.uid);
        Ok(self.core.svc.get_stat_fits_dmg_applied(
            &mut CseqMap::new(),
            &self.core.u_data,
            u_fleet.iter_fits(),
            item_kinds,
            time_options,
            crit_options,
            projectee_uid,
        ))
    }
}
