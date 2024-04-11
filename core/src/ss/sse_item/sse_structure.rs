use crate::{
    defs::{EItemId, SsFitId, SsItemId},
    ss::{
        item::{SsItem, SsStructure},
        item_info::SsStructureInfo,
        SolarSystem,
    },
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_fit_structure_info(&self, fit_id: &SsFitId) -> Result<SsStructureInfo> {
        self.get_fit_structure(fit_id).map(|v| v.into())
    }
    pub fn set_fit_structure(&mut self, fit_id: SsFitId, a_item_id: EItemId, state: bool) -> Result<SsStructureInfo> {
        match self.remove_fit_structure(&fit_id) {
            Ok(_) => (),
            // Suppress ItemTypeNotFound error, since this method is supposed to be used
            // even when no structure is set
            Err(e) => match e.kind {
                ErrorKind::SsItemTypeNotFound(_) => (),
                _ => return Err(e),
            },
        };
        let item_id = self.items.alloc_item_id()?;
        let structure = SsStructure::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = SsStructureInfo::from(&structure);
        let item = SsItem::Structure(structure);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_structure_state(&mut self, item_id: &SsItemId, state: bool) -> Result<()> {
        self.items.get_structure_mut(item_id)?.set_bool_state(state);
        Ok(())
    }
    pub fn set_fit_structure_state(&mut self, fit_id: &SsFitId, state: bool) -> Result<()> {
        self.get_fit_structure_mut(fit_id)?.set_bool_state(state);
        Ok(())
    }
    pub fn remove_fit_structure(&mut self, fit_id: &SsFitId) -> Result<()> {
        let item_id = self.get_fit_structure_id(fit_id)?;
        self.remove_item(&item_id)
    }
    // Non-public
    fn get_fit_structure_id(&self, fit_id: &SsFitId) -> Result<SsItemId> {
        self.fits
            .get_fit(fit_id)?
            .structure
            .ok_or_else(|| Error::new(ErrorKind::SsItemTypeNotFound(SsStructure::get_name())))
    }
    fn get_fit_structure(&self, fit_id: &SsFitId) -> Result<&SsStructure> {
        let item_id = self.get_fit_structure_id(fit_id)?;
        self.items.get_structure(&item_id)
    }
    fn get_fit_structure_mut(&mut self, fit_id: &SsFitId) -> Result<&mut SsStructure> {
        let item_id = self.get_fit_structure_id(fit_id)?;
        self.items.get_structure_mut(&item_id)
    }
}
