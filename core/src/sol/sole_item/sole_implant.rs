use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::{
        item::{SolImplant, SolItem},
        item_info::SolImplantInfo,
        SolView, SolarSystem,
    },
    util::Result,
};

impl SolarSystem {
    // Public
    pub fn get_implant_info(&self, item_id: &SolItemId) -> Result<SolImplantInfo> {
        Ok(self.items.get_implant(item_id)?.into())
    }
    pub fn get_fit_implant_infos(&self, fit_id: &SolFitId) -> Result<Vec<SolImplantInfo>> {
        let fit = self.fits.get_fit(fit_id)?;
        let implant_infos = fit
            .implants
            .iter()
            .map(|v| self.items.get_implant(v).unwrap().into())
            .collect();
        Ok(implant_infos)
    }
    pub fn add_implant(&mut self, fit_id: SolFitId, a_item_id: EItemId, state: bool) -> Result<SolImplantInfo> {
        let item_id = self.items.alloc_item_id()?;
        let implant = SolImplant::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = SolImplantInfo::from(&implant);
        let item = SolItem::Implant(implant);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_implant_state(&mut self, item_id: &SolItemId, state: bool) -> Result<()> {
        let implant = self.items.get_implant_mut(item_id)?;
        let old_state = implant.state;
        implant.set_bool_state(state);
        let new_state = implant.state;
        if new_state != old_state {
            let item = self.items.get_item(item_id).unwrap();
            self.svcs.switch_item_state(
                &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
                item,
                old_state,
                new_state,
            );
        }
        Ok(())
    }
}
