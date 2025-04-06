use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, SolarSystem},
};

impl SolarSystem {
    pub fn remove_rig(&mut self, item_id: &ItemId) -> Result<(), RemoveRigError> {
        let item = self.uad.items.get_by_id(item_id)?;
        let rig = item.get_rig()?;
        self.svc.remove_item(&self.uad, item);
        let fit = self.uad.fits.get_fit_mut(&rig.get_fit_id()).unwrap();
        fit.rigs.remove(item_id);
        self.uad.items.remove_by_id(item_id);
        Ok(())
    }
}

#[derive(Debug)]
pub enum RemoveRigError {
    ItemNotFound(ItemFoundError),
    ItemIsNotRig(ItemKindMatchError),
}
impl std::error::Error for RemoveRigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotRig(e) => Some(e),
        }
    }
}
impl std::fmt::Display for RemoveRigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotRig(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for RemoveRigError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for RemoveRigError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotRig(error)
    }
}
