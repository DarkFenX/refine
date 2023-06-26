use crate::{
    defs::{ReeInt, SsFitId, SsItemId},
    ss::SolarSystem,
    ssi, ssn,
    util::Result,
};

impl SolarSystem {
    // Public
    pub fn get_implant_info(&self, item_id: &SsItemId) -> Result<ssn::SsImplantInfo> {
        Ok(self.items.get_implant(item_id)?.into())
    }
    pub fn get_fit_implant_infos(&self, fit_id: &SsFitId) -> Result<Vec<ssn::SsImplantInfo>> {
        let fit = self.fits.get_fit(fit_id)?;
        let implant_infos = fit
            .implants
            .iter()
            .map(|v| self.items.get_implant(v).unwrap().into())
            .collect();
        Ok(implant_infos)
    }
    pub fn add_implant(&mut self, fit_id: SsFitId, a_item_id: ReeInt, state: bool) -> Result<ssn::SsImplantInfo> {
        let item_id = self.items.alloc_item_id()?;
        let implant = ssi::SsImplant::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = ssn::SsImplantInfo::from(&implant);
        let item = ssi::SsItem::Implant(implant);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_implant_state(&mut self, item_id: &SsItemId, state: bool) -> Result<()> {
        self.items.get_implant_mut(item_id)?.set_bool_state(state);
        Ok(())
    }
}
