use crate::{
    defs::{ReeInt, SsFitId, SsItemId},
    ss::SolarSystem,
    ssi, ssn,
    util::Result,
};

impl SolarSystem {
    // Public
    pub fn get_rig_info(&self, item_id: &SsItemId) -> Result<ssn::SsRigInfo> {
        Ok(self.items.get_rig(item_id)?.into())
    }
    pub fn get_fit_rig_infos(&self, fit_id: &SsFitId) -> Result<Vec<ssn::SsRigInfo>> {
        let fit = self.fits.get_fit(fit_id)?;
        let rig_infos = fit.rigs.iter().map(|v| self.items.get_rig(v).unwrap().into()).collect();
        Ok(rig_infos)
    }
    pub fn add_rig(&mut self, fit_id: SsFitId, a_item_id: ReeInt, state: bool) -> Result<ssn::SsRigInfo> {
        let item_id = self.items.alloc_item_id()?;
        let rig = ssi::SsRig::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = ssn::SsRigInfo::from(&rig);
        let item = ssi::SsItem::Rig(rig);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_rig_state(&mut self, item_id: &SsItemId, state: bool) -> Result<()> {
        self.items.get_rig_mut(item_id)?.set_bool_state(state);
        Ok(())
    }
}
