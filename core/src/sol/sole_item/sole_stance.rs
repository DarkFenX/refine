use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::{
        item::{SolItem, SolStance},
        item_info::SolStanceInfo,
        SolarSystem,
    },
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_fit_stance_info(&self, fit_id: &SolFitId) -> Result<SolStanceInfo> {
        self.get_fit_stance(fit_id).map(|v| v.into())
    }
    pub fn set_fit_stance(&mut self, fit_id: SolFitId, a_item_id: EItemId, state: bool) -> Result<SolStanceInfo> {
        match self.remove_fit_stance(&fit_id) {
            Ok(_) => (),
            // Suppress SolItemKindNotFound error, since this method is supposed to be used even
            // when no stance is set
            Err(e) => match e.kind {
                ErrorKind::SolItemKindNotFound(_) => (),
                _ => return Err(e),
            },
        };
        let item_id = self.items.alloc_item_id()?;
        let stance = SolStance::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = SolStanceInfo::from(&stance);
        let item = SolItem::Stance(stance);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_stance_state(&mut self, item_id: &SolItemId, state: bool) -> Result<()> {
        self.items.get_stance_mut(item_id)?.set_bool_state(state);
        Ok(())
    }
    pub fn set_fit_stance_state(&mut self, fit_id: &SolFitId, state: bool) -> Result<()> {
        self.get_fit_stance_mut(fit_id)?.set_bool_state(state);
        Ok(())
    }
    pub fn remove_fit_stance(&mut self, fit_id: &SolFitId) -> Result<()> {
        let item_id = self.get_fit_stance_id(fit_id)?;
        self.remove_item(&item_id)
    }
    // Non-public
    fn get_fit_stance_id(&self, fit_id: &SolFitId) -> Result<SolItemId> {
        self.fits
            .get_fit(fit_id)?
            .stance
            .ok_or_else(|| Error::new(ErrorKind::SolItemKindNotFound(SolStance::get_name())))
    }
    fn get_fit_stance(&self, fit_id: &SolFitId) -> Result<&SolStance> {
        let item_id = self.get_fit_stance_id(fit_id)?;
        self.items.get_stance(&item_id)
    }
    fn get_fit_stance_mut(&mut self, fit_id: &SolFitId) -> Result<&mut SolStance> {
        let item_id = self.get_fit_stance_id(fit_id)?;
        self.items.get_stance_mut(&item_id)
    }
}
