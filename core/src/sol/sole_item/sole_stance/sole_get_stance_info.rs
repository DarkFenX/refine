use crate::{
    err::basic::ItemKindMatchError,
    sol::{ItemKey, SolarSystem, info::StanceInfo},
};

impl SolarSystem {
    pub(in crate::sol) fn get_stance_info_internal(&self, item_key: ItemKey) -> Result<StanceInfo, ItemKindMatchError> {
        let stance = self.uad.items.get(item_key).get_stance()?;
        Ok(StanceInfo::from_stance(&self.uad, stance))
    }
}
