use crate::{
    defs::{EItemId, SsFitId, SsItemId},
    ss::{
        info::SsStanceInfo,
        item::{SsItem, SsStance},
        SolarSystem,
    },
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_fit_stance_info(&self, fit_id: &SsFitId) -> Result<SsStanceInfo> {
        self.get_fit_stance(fit_id).map(|v| v.into())
    }
    pub fn set_fit_stance(&mut self, fit_id: SsFitId, a_item_id: EItemId, state: bool) -> Result<SsStanceInfo> {
        match self.remove_fit_stance(&fit_id) {
            Ok(_) => (),
            // Suppress ItemTypeNotFound error, since this method is supposed to be used
            // even when no stance is set
            Err(e) => match e.kind {
                ErrorKind::SsItemTypeNotFound(_) => (),
                _ => return Err(e),
            },
        };
        let item_id = self.items.alloc_item_id()?;
        let stance = SsStance::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = SsStanceInfo::from(&stance);
        let item = SsItem::Stance(stance);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_stance_state(&mut self, item_id: &SsItemId, state: bool) -> Result<()> {
        self.items.get_stance_mut(item_id)?.set_bool_state(state);
        Ok(())
    }
    pub fn remove_fit_stance(&mut self, fit_id: &SsFitId) -> Result<()> {
        let item_id = self.get_fit_stance_id(fit_id)?;
        self.remove_item(&item_id)
    }
    // Non-public
    fn get_fit_stance_id(&self, fit_id: &SsFitId) -> Result<SsItemId> {
        self.fits
            .get_fit(fit_id)?
            .stance
            .ok_or_else(|| Error::new(ErrorKind::SsItemTypeNotFound(SsStance::get_name())))
    }
    fn get_fit_stance(&self, fit_id: &SsFitId) -> Result<&SsStance> {
        let item_id = self.get_fit_stance_id(fit_id)?;
        self.items.get_stance(&item_id)
    }
}
