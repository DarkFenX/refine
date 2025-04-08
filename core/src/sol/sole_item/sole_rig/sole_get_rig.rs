use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, ItemKey, SolarSystem, info::RigInfo},
};

impl SolarSystem {
    pub fn get_rig(&self, item_id: &ItemId) -> Result<RigInfo, GetRigError> {
        let item_key = self.uad.items.key_by_id_err(item_id)?;
        Ok(self.get_rig_internal(item_key)?)
    }
    pub(in crate::sol) fn get_rig_internal(&self, item_key: ItemKey) -> Result<RigInfo, ItemKindMatchError> {
        let rig = self.uad.items.get(item_key).get_rig()?;
        Ok(RigInfo::from_rig(&self.uad, rig))
    }
}

#[derive(Debug)]
pub enum GetRigError {
    ItemNotFound(ItemFoundError),
    ItemIsNotRig(ItemKindMatchError),
}
impl std::error::Error for GetRigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotRig(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetRigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotRig(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for GetRigError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for GetRigError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotRig(error)
    }
}
