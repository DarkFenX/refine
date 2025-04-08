use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn remove_stance(&mut self, item_id: &ItemId) -> Result<(), RemoveStanceError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.remove_stance_internal(item_key)?)
    }
    pub(in crate::sol) fn remove_stance_internal(&mut self, item_key: ItemKey) -> Result<(), ItemKindMatchError> {
        let item = self.uad.items.get(item_key);
        let stance = item.get_stance()?;
        self.svc.remove_item(&self.uad, item_key, item);
        let fit = self.uad.fits.get_mut(stance.get_fit_key());
        fit.stance = None;
        self.uad.items.remove(item_key);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveStanceError {
    ItemNotFound(ItemFoundError),
    ItemIsNotStance(ItemKindMatchError),
}
impl std::error::Error for RemoveStanceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotStance(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveStanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotStance(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for RemoveStanceError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for RemoveStanceError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotStance(error)
    }
}
