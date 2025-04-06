use crate::{
    err::basic::{ItemFoundError, ItemKindMatchError},
    sol::{ItemId, SolarSystem, info::ImplantInfo},
};

impl SolarSystem {
    pub fn get_implant(&self, item_id: &ItemId) -> Result<ImplantInfo, GetImplantError> {
        let implant = self.uad.items.get_by_id(item_id)?.get_implant()?;
        Ok(ImplantInfo::from(implant))
    }
}

#[derive(Debug)]
pub enum GetImplantError {
    ItemNotFound(ItemFoundError),
    ItemIsNotImplant(ItemKindMatchError),
}
impl std::error::Error for GetImplantError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemIsNotImplant(e) => Some(e),
        }
    }
}
impl std::fmt::Display for GetImplantError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemIsNotImplant(e) => e.fmt(f),
        }
    }
}
impl From<ItemFoundError> for GetImplantError {
    fn from(error: ItemFoundError) -> Self {
        Self::ItemNotFound(error)
    }
}
impl From<ItemKindMatchError> for GetImplantError {
    fn from(error: ItemKindMatchError) -> Self {
        Self::ItemIsNotImplant(error)
    }
}
