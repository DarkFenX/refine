use crate::{
    ad::{AAttr, AAttrId, ABuff, ABuffId, AEffect, AEffectId, AItem, AItemId, AMuta},
    util::RMap,
};

/// Adapted data storage.
pub struct AData {
    pub items: RMap<AItemId, AItem>,
    pub attrs: RMap<AAttrId, AAttr>,
    pub mutas: RMap<AItemId, AMuta>,
    pub effects: RMap<AEffectId, AEffect>,
    pub buffs: RMap<ABuffId, ABuff>,
}
impl AData {
    pub(crate) fn new() -> Self {
        Self {
            items: RMap::new(),
            attrs: RMap::new(),
            mutas: RMap::new(),
            effects: RMap::new(),
            buffs: RMap::new(),
        }
    }
}
