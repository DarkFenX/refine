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

#[derive(Debug)]
pub enum RemoveSubsystemError {
    ItemNotFound(ItemFoundError),
    ItemIsNotSubsystem(ItemKindMatchError),
}
impl std::error::Error for RemoveSubsystemError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotSubsystem(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveSubsystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotSubsystem(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for RemoveSubsystemError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for RemoveSubsystemError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotSubsystem(error)
    }
}
