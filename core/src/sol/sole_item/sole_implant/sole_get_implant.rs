use crate::{
    defs::SolItemId,
    sol::{
        err::basic::{ItemFoundError, ItemKindMatchError},
        item_info::SolImplantInfo,
        SolarSystem,
    },
};

impl SolarSystem {
    pub fn get_implant(&self, item_id: &SolItemId) -> Result<SolImplantInfo, GetImplantError> {
        let implant = self.items.get_item(item_id)?.get_implant()?;
        Ok(SolImplantInfo::from(implant))
    }
}

#[derive(Debug)]
pub enum GetImplantError {
    ItemNotFound(ItemFoundError),
    ItemIsNotImplant(ItemKindMatchError),
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
