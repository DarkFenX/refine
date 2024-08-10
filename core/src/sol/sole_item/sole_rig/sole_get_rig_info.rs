use crate::{
    defs::SolItemId,
    err::{ItemFoundError, ItemKindMatchError},
    sol::{item_info::SolRigInfo, SolarSystem},
};

impl SolarSystem {
    pub fn get_rig_info(&self, item_id: &SolItemId) -> Result<SolRigInfo, GetRigInfoError> {
        let rig = self.items.get_item(item_id)?.get_rig()?;
        Ok(SolRigInfo::from(rig))
    }
}

#[derive(Debug)]
pub enum GetRigInfoError {
    ItemNotFound(ItemFoundError),
    ItemIsNotRig(ItemKindMatchError),
}
impl From<ItemFoundError> for GetRigInfoError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for GetRigInfoError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotRig(error)
    }
}
impl std::error::Error for GetRigInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotRig(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetRigInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotRig(e) => e.fmt(f),
        }
    }
}
