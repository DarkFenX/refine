use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, FitKey, ItemKey, ItemTypeId, SolarSystem,
        info::SubsystemInfo,
        uad::item::{UadItem, UadSubsystem},
    },
};

impl SolarSystem {
    pub fn add_subsystem(
        &mut self,
        fit_id: &FitId,
        type_id: ItemTypeId,
        state: bool,
    ) -> Result<SubsystemInfo, AddSubsystemError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        let item_key = self.add_subsystem_internal(fit_key, type_id, state);
        Ok(self.get_subsystem_info_internal(item_key).unwrap())
    }
    pub(in crate::sol) fn add_subsystem_internal(
        &mut self,
        fit_key: FitKey,
        type_id: ItemTypeId,
        state: bool,
    ) -> ItemKey {
        let item_id = self.uad.items.alloc_id();
        let subsystem = UadSubsystem::new(&self.uad.src, item_id, type_id, fit_key, state);
        let item = UadItem::Subsystem(subsystem);
        let item_key = self.uad.items.add(item);
        let fit = self.uad.fits.get_mut(fit_key);
        fit.subsystems.insert(item_key);
        self.add_item_key_to_svc(item_key);
        item_key
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AddSubsystemError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
