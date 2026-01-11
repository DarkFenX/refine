use crate::cacher_json::data::{
    abil::CAbil, attr::CAttr, buff::CBuff, effect::CEffect, item::CItem, item_list::CItemList, muta::CMuta,
};

#[derive(serde::Serialize, serde::Deserialize)]
pub(in crate::cacher_json) struct CData {
    items: Vec<CItem>,
    attrs: Vec<CAttr>,
    mutas: Vec<CMuta>,
    effects: Vec<CEffect>,
    buffs: Vec<CBuff>,
    abils: Vec<CAbil>,
    item_lists: Vec<CItemList>,
}
impl CData {
    pub(in crate::cacher_json) fn from_adapted(a_data: &rc::ad::AData) -> Self {
        Self {
            items: a_data.items.iter().map(CItem::from_adapted).collect(),
            attrs: a_data.attrs.iter().map(CAttr::from_adapted).collect(),
            mutas: a_data.mutas.iter().map(CMuta::from_adapted).collect(),
            effects: a_data.effects.iter().map(CEffect::from_adapted).collect(),
            buffs: a_data.buffs.iter().map(CBuff::from_adapted).collect(),
            abils: a_data.abils.iter().map(CAbil::from_adapted).collect(),
            item_lists: a_data.item_lists.iter().map(CItemList::from_adapted).collect(),
        }
    }
    pub(in crate::cacher_json) fn into_adapted(self) -> rc::ad::AData {
        rc::ad::AData {
            items: self.items.into_iter().map(|v| v.into_adapted()).collect(),
            attrs: self.attrs.into_iter().map(|v| v.into_adapted()).collect(),
            mutas: self.mutas.into_iter().map(|v| v.into_adapted()).collect(),
            effects: self.effects.into_iter().map(|v| v.into_adapted()).collect(),
            buffs: self.buffs.into_iter().map(|v| v.into_adapted()).collect(),
            abils: self.abils.into_iter().map(|v| v.into_adapted()).collect(),
            item_lists: self.item_lists.into_iter().map(|v| v.into_adapted()).collect(),
        }
    }
}
