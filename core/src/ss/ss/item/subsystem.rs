use crate::{
    defs::{EItemId, SsFitId, SsItemId},
    ss::{
        info::SsSubsystemInfo,
        item::{SsItem, SsSubsystem},
        SolarSystem,
    },
    util::Result,
};

impl SolarSystem {
    // Public
    pub fn get_subsystem_info(&self, item_id: &SsItemId) -> Result<SsSubsystemInfo> {
        Ok(self.items.get_subsystem(item_id)?.into())
    }
    pub fn get_fit_subsystem_infos(&self, fit_id: &SsFitId) -> Result<Vec<SsSubsystemInfo>> {
        let fit = self.fits.get_fit(fit_id)?;
        let subsystem_infos = fit
            .subsystems
            .iter()
            .map(|v| self.items.get_subsystem(v).unwrap().into())
            .collect();
        Ok(subsystem_infos)
    }
    pub fn add_subsystem(&mut self, fit_id: SsFitId, a_item_id: EItemId, state: bool) -> Result<SsSubsystemInfo> {
        let fit = self.fits.get_fit_mut(&fit_id)?;
        let item_id = self.items.alloc_item_id()?;
        let subsystem = SsSubsystem::new(&self.src, item_id, fit_id, fit.ship, a_item_id, state);
        let info = SsSubsystemInfo::from(&subsystem);
        let item = SsItem::Subsystem(subsystem);
        fit.add_item(&item);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_subsystem_state(&mut self, item_id: &SsItemId, state: bool) -> Result<()> {
        self.items.get_subsystem_mut(item_id)?.set_bool_state(state);
        Ok(())
    }
}
