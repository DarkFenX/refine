use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem},
};

impl SolarSystem {
    pub fn remove_rig(&mut self, item_id: &ItemId) -> Result<(), RemoveRigError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.remove_rig_internal(item_key)?)
    }
    pub(in crate::sol) fn remove_rig_internal(&mut self, item_key: ItemKey) -> Result<(), ItemKindMatchError> {
        let item = self.uad.items.get(item_key);
        let rig = item.get_rig()?;
        self.svc.remove_item(&self.uad, item_key, item);
        let fit = self.uad.fits.get_mut(rig.get_fit_key());
        fit.rigs.remove(&item_key);
        self.uad.items.remove(item_key);
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RemoveRigError {
    #[error("{0}")]
    ItemNotFound(#[from] ItemFoundError),
    #[error("{0}")]
    ItemIsNotRig(#[from] ItemKindMatchError),
}
