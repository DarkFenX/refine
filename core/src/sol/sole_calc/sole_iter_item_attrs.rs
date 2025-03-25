use crate::{
    err::basic::{ItemFoundError, ItemLoadedError},
    sol::{
        AttrId, ItemId, SolarSystem,
        svc::calc::{CalcAttrVal, LoadedItemFoundError},
    },
};

impl SolarSystem {
    pub fn iter_item_attrs(
        &mut self,
        item_id: &ItemId,
    ) -> Result<impl ExactSizeIterator<Item = (AttrId, CalcAttrVal)>, IterItemAttrsError> {
        let attrs = self.svc.calc.iter_item_attr_vals(&self.uad, item_id)?;
        Ok(attrs)
    }
}

#[derive(Debug)]
pub enum IterItemAttrsError {
    ItemNotFound(ItemFoundError),
    ItemNotLoaded(ItemLoadedError),
}
impl std::error::Error for IterItemAttrsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemNotLoaded(e) => Some(e),
        }
    }
}
impl std::fmt::Display for IterItemAttrsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemNotLoaded(e) => e.fmt(f),
        }
    }
}
impl From<LoadedItemFoundError> for IterItemAttrsError {
    fn from(error: LoadedItemFoundError) -> Self {
        match error {
            LoadedItemFoundError::ItemNotFound(e) => Self::ItemNotFound(e),
            LoadedItemFoundError::ItemNotLoaded(e) => Self::ItemNotLoaded(e),
        }
    }
}
