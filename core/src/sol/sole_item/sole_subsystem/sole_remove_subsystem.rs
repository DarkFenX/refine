use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn remove_subsystem(&mut self, item_id: &ItemId) -> Result<(), RemoveSubsystemError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.remove_subsystem_internal(item_key)?)
    }
    pub(in crate::sol) fn remove_subsystem_internal(&mut self, item_key: ItemKey) -> Result<(), ItemKindMatchError> {
        let item = self.uad.items.get(item_key);
        let subsystem = item.get_subsystem()?;
        self.svc.remove_item(&self.uad, item_key, item);
        let fit = self.uad.fits.get_mut(subsystem.get_fit_key());
        fit.subsystems.remove(&item_key);
        self.uad.items.remove(item_key);
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RemoveSubsystemError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotSubsystem(#[from] ItemKindMatchError),
}
