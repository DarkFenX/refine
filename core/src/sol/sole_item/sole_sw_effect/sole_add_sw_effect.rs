use crate::{
    defs::EItemId,
    err::basic::ItemAllocError,
    sol::{
        item::{SolItem, SolSwEffect},
        item_info::SolSwEffectInfo,
        SolarSystem,
    },
};

impl SolarSystem {
    pub fn add_sw_effect(&mut self, a_item_id: EItemId, state: bool) -> Result<SolSwEffectInfo, AddSwEffectError> {
        let item_id = self.items.alloc_item_id()?;
        let sw_effect = SolSwEffect::new(&self.src, item_id, a_item_id, state);
        let info = SolSwEffectInfo::from(&sw_effect);
        let item = SolItem::SwEffect(sw_effect);
        self.sw_effects.insert(item_id);
        self.items.add_item(item);
        self.add_item_id_to_svcs(&item_id);
        Ok(info)
    }
}

#[derive(Debug)]
pub enum AddSwEffectError {
    ItemIdAllocFailed(ItemAllocError),
}
impl From<ItemAllocError> for AddSwEffectError {
    fn from(error: ItemAllocError) -> Self {
        Self::ItemIdAllocFailed(error)
    }
}
impl std::error::Error for AddSwEffectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ItemIdAllocFailed(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddSwEffectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemIdAllocFailed(e) => e.fmt(f),
        }
    }
}
