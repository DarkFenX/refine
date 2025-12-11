use crate::{
    ad::{
        AAbil, AAbilId, AAttr, AAttrId, ABuff, ABuffId, AEffect, AEffectId, AItem, AItemId, AItemList, AItemListId,
        AMuta,
    },
    util::RMap,
};

#[derive(Clone)]
pub struct AData {
    pub items: RMap<AItemId, AItem>,
    pub attrs: RMap<AAttrId, AAttr>,
    pub mutas: RMap<AItemId, AMuta>,
    pub effects: RMap<AEffectId, AEffect>,
    pub buffs: RMap<ABuffId, ABuff>,
    pub abils: RMap<AAbilId, AAbil>,
    pub item_lists: RMap<AItemListId, AItemList>,
}
impl AData {
    pub(crate) fn new() -> Self {
        Self {
            items: RMap::new(),
            attrs: RMap::new(),
            mutas: RMap::new(),
            effects: RMap::new(),
            buffs: RMap::new(),
            abils: RMap::new(),
            item_lists: RMap::new(),
        }
    }
}
