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

#[derive(thiserror::Error, Debug)]
pub enum RemoveStanceError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotStance(#[from] ItemKindMatchError),
}
