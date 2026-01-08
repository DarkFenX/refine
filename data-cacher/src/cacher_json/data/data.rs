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
            items: a_data.items.values().map(CItem::from_adapted).collect(),
            attrs: a_data.attrs.values().map(CAttr::from_adapted).collect(),
            mutas: a_data.mutas.values().map(CMuta::from_adapted).collect(),
            effects: a_data.effects.values().map(CEffect::from_adapted).collect(),
            buffs: a_data.buffs.values().map(CBuff::from_adapted).collect(),
            abils: a_data.abils.values().map(CAbil::from_adapted).collect(),
            item_lists: a_data.item_lists.values().map(CItemList::from_adapted).collect(),
        }
    }
    pub(in crate::cacher_json) fn into_adapted(self) -> rc::ad::AData {
        rc::ad::AData {
            items: self
                .items
                .into_iter()
                .map(|v| {
                    let a_item = v.into_adapted();
                    (a_item.id, a_item)
                })
                .collect(),
            attrs: self
                .attrs
                .into_iter()
                .map(|v| {
                    let a_attr = v.into_adapted();
                    (a_attr.id, a_attr)
                })
                .collect(),
            mutas: self
                .mutas
                .into_iter()
                .map(|v| {
                    let a_muta = v.into_adapted();
                    (a_muta.id, a_muta)
                })
                .collect(),
            effects: self
                .effects
                .into_iter()
                .map(|v| {
                    let a_effect = v.into_adapted();
                    (a_effect.id, a_effect)
                })
                .collect(),
            buffs: self
                .buffs
                .into_iter()
                .map(|v| {
                    let a_buffs = v.into_adapted();
                    (a_buffs.id, a_buffs)
                })
                .collect(),
            abils: self
                .abils
                .into_iter()
                .map(|v| {
                    let a_abil = v.into_adapted();
                    (a_abil.id, a_abil)
                })
                .collect(),
            item_lists: self
                .item_lists
                .into_iter()
                .map(|v| {
                    let a_item_list = v.into_adapted();
                    (a_item_list.id, a_item_list)
                })
                .collect(),
        }
    }
}
