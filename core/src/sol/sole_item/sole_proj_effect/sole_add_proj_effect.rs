use crate::{
    defs::EItemId,
    sol::{
        err::basic::ItemAllocError,
        item::{SolItem, SolProjEffect},
        item_info::SolProjEffectInfo,
        SolarSystem,
    },
};

impl SolarSystem {
    pub fn add_proj_effect(
        &mut self,
        a_item_id: EItemId,
        state: bool,
    ) -> Result<SolProjEffectInfo, AddProjEffectError> {
        let item_id = self.items.alloc_item_id()?;
        let proj_effect = SolProjEffect::new(&self.src, item_id, a_item_id, state);
        let info = SolProjEffectInfo::from(&proj_effect);
        let item = SolItem::ProjEffect(proj_effect);
        self.proj_effects.insert(item_id);
        self.items.add_item(item);
        self.add_item_id_to_svcs(&item_id);
        Ok(info)
    }
}

#[derive(Debug)]
pub enum AddProjEffectError {
    ItemIdAllocFailed(ItemAllocError),
}
impl From<ItemAllocError> for AddProjEffectError {
    fn from(error: ItemAllocError) -> Self {
        Self::ItemIdAllocFailed(error)
    }
}
impl std::error::Error for AddProjEffectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemIdAllocFailed(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddProjEffectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemIdAllocFailed(e) => e.fmt(f),
        }
    }
}
