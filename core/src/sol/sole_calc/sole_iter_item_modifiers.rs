use crate::{
    defs::{EAttrId, SolItemId},
    err::basic::{ItemFoundError, ItemLoadedError},
    sol::{
        svc::{err::LoadedItemFoundError, SolModificationInfo},
        SolView, SolarSystem,
    },
};

impl SolarSystem {
    pub fn iter_item_modifiers(
        &mut self,
        item_id: &SolItemId,
    ) -> Result<impl ExactSizeIterator<Item = (EAttrId, Vec<SolModificationInfo>)>, IterItemModifiersError> {
        let modifiers = self
            .svcs
            .calc_iter_item_mods(&SolView::new(&self.src, &self.fleets, &self.fits, &self.items), item_id)?;
        Ok(modifiers)
    }
}

#[derive(Debug)]
pub enum IterItemModifiersError {
    ItemNotFound(ItemFoundError),
    ItemNotLoaded(ItemLoadedError),
}
impl From<LoadedItemFoundError> for IterItemModifiersError {
    fn from(error: LoadedItemFoundError) -> Self {
        match error {
            LoadedItemFoundError::ItemNotFound(e) => Self::ItemNotFound(e),
            LoadedItemFoundError::ItemNotLoaded(e) => Self::ItemNotLoaded(e),
        }
    }
}
impl std::error::Error for IterItemModifiersError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemNotFound(e) => Some(e),
            Self::ItemNotLoaded(e) => Some(e),
        }
    }
}
impl std::fmt::Display for IterItemModifiersError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(e) => e.fmt(f),
            Self::ItemNotLoaded(e) => e.fmt(f),
        }
    }
}
