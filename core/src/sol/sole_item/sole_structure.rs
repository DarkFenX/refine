use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::{
        item::{SolItem, SolStructure},
        item_info::SolStructureInfo,
        SolarSystem,
    },
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_fit_structure_info(&self, fit_id: &SolFitId) -> Result<SolStructureInfo> {
        self.get_fit_structure(fit_id).map(|v| v.into())
    }
    pub fn set_fit_structure(&mut self, fit_id: SolFitId, a_item_id: EItemId, state: bool) -> Result<SolStructureInfo> {
        match self.remove_fit_structure(&fit_id) {
            Ok(_) => (),
            // Suppress SolItemKindNotFound error, since this method is supposed to be used even
            // when no structure is set
            Err(e) => match e.kind {
                ErrorKind::SolItemKindNotFound(_) => (),
                _ => return Err(e),
            },
        };
        let item_id = self.items.alloc_item_id()?;
        let structure = SolStructure::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = SolStructureInfo::from(&structure);
        let item = SolItem::Structure(structure);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_structure_state(&mut self, item_id: &SolItemId, state: bool) -> Result<()> {
        self.items.get_structure_mut(item_id)?.set_bool_state(state);
        Ok(())
    }
    pub fn set_fit_structure_state(&mut self, fit_id: &SolFitId, state: bool) -> Result<()> {
        self.get_fit_structure_mut(fit_id)?.set_bool_state(state);
        Ok(())
    }
    pub fn remove_fit_structure(&mut self, fit_id: &SolFitId) -> Result<()> {
        let item_id = self.get_fit_structure_id(fit_id)?;
        self.remove_item(&item_id)
    }
    // Non-public
    fn get_fit_structure_id(&self, fit_id: &SolFitId) -> Result<SolItemId> {
        self.fits
            .get_fit(fit_id)?
            .structure
            .ok_or_else(|| Error::new(ErrorKind::SolItemKindNotFound(SolStructure::get_name())))
    }
    fn get_fit_structure(&self, fit_id: &SolFitId) -> Result<&SolStructure> {
        let item_id = self.get_fit_structure_id(fit_id)?;
        self.items.get_structure(&item_id)
    }
    fn get_fit_structure_mut(&mut self, fit_id: &SolFitId) -> Result<&mut SolStructure> {
        let item_id = self.get_fit_structure_id(fit_id)?;
        self.items.get_structure_mut(&item_id)
    }
}
