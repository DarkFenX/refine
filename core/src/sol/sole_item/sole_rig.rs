use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::{
        item::{SolItem, SolRig},
        item_info::SolRigInfo,
        SolView, SolarSystem,
    },
    util::Result,
};

impl SolarSystem {
    // Public
    pub fn get_rig_info(&self, item_id: &SolItemId) -> Result<SolRigInfo> {
        Ok(self.items.get_rig(item_id)?.into())
    }
    pub fn get_fit_rig_infos(&self, fit_id: &SolFitId) -> Result<Vec<SolRigInfo>> {
        let fit = self.fits.get_fit(fit_id)?;
        let rig_infos = fit.rigs.iter().map(|v| self.items.get_rig(v).unwrap().into()).collect();
        Ok(rig_infos)
    }
    pub fn add_rig(&mut self, fit_id: SolFitId, a_item_id: EItemId, state: bool) -> Result<SolRigInfo> {
        let item_id = self.items.alloc_item_id()?;
        let rig = SolRig::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = SolRigInfo::from(&rig);
        let item = SolItem::Rig(rig);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_rig_state(&mut self, item_id: &SolItemId, state: bool) -> Result<()> {
        let rig = self.items.get_rig_mut(item_id)?;
        let old_state = rig.state;
        rig.set_bool_state(state);
        let new_state = rig.state;
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
