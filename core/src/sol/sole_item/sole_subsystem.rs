use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::{
        item::{SolItem, SolSubsystem},
        item_info::SolSubsystemInfo,
        SolarSystem,
    },
    util::Result,
};

impl SolarSystem {
    // Public
    pub fn get_subsystem_info(&self, item_id: &SolItemId) -> Result<SolSubsystemInfo> {
        Ok(self.items.get_subsystem(item_id)?.into())
    }
    pub fn get_fit_subsystem_infos(&self, fit_id: &SolFitId) -> Result<Vec<SolSubsystemInfo>> {
        let fit = self.fits.get_fit(fit_id)?;
        let subsystem_infos = fit
            .subsystems
            .iter()
            .map(|v| self.items.get_subsystem(v).unwrap().into())
            .collect();
        Ok(subsystem_infos)
    }
    pub fn add_subsystem(&mut self, fit_id: SolFitId, a_item_id: EItemId, state: bool) -> Result<SolSubsystemInfo> {
        let item_id = self.items.alloc_item_id()?;
        let subsystem = SolSubsystem::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = SolSubsystemInfo::from(&subsystem);
        let item = SolItem::Subsystem(subsystem);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_subsystem_state(&mut self, item_id: &SolItemId, state: bool) -> Result<()> {
        self.items.get_subsystem_mut(item_id)?.set_bool_state(state);
        Ok(())
    }
}
