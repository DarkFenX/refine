use crate::cacher_json::data::{CAbil, CAttr, CBuff, CEffect, CItem, CItemList, CMuta};

#[derive(serde::Serialize, serde::Deserialize)]
pub(in crate::cacher_json) struct CData {
    pub(in crate::cacher_json) items: Vec<CItem>,
    pub(in crate::cacher_json) attrs: Vec<CAttr>,
    pub(in crate::cacher_json) mutas: Vec<CMuta>,
    pub(in crate::cacher_json) effects: Vec<CEffect>,
    pub(in crate::cacher_json) buffs: Vec<CBuff>,
    pub(in crate::cacher_json) abils: Vec<CAbil>,
    pub(in crate::cacher_json) item_lists: Vec<CItemList>,
}
impl From<&rc::ad::AData> for CData {
    fn from(a_data: &rc::ad::AData) -> Self {
        Self {
            items: a_data.items.values().map(Into::into).collect(),
            attrs: a_data.attrs.values().map(Into::into).collect(),
            mutas: a_data.mutas.values().map(Into::into).collect(),
            effects: a_data.effects.values().map(Into::into).collect(),
            buffs: a_data.buffs.values().map(Into::into).collect(),
            abils: a_data.abils.values().map(Into::into).collect(),
            item_lists: a_data.item_lists.values().map(Into::into).collect(),
        }
    }
}
impl From<&CData> for rc::ad::AData {
    fn from(c_data: &CData) -> Self {
        Self {
            items: c_data
                .items
                .iter()
                .map(|v| {
                    let a_item = rc::ad::AItem::from(v);
                    (a_item.id, a_item)
                })
                .collect(),
            attrs: c_data
                .attrs
                .iter()
                .map(|v| {
                    let a_attr = rc::ad::AAttr::from(v);
                    (a_attr.id, a_attr)
                })
                .collect(),
            mutas: c_data
                .mutas
                .iter()
                .map(|v| {
                    let a_muta = rc::ad::AMuta::from(v);
                    (a_muta.id, a_muta)
                })
                .collect(),
            effects: c_data
                .effects
                .iter()
                .map(|v| {
                    let a_effect = rc::ad::AEffect::from(v);
                    (a_effect.id, a_effect)
                })
                .collect(),
            buffs: c_data
                .buffs
                .iter()
                .map(|v| {
                    let a_buff = rc::ad::ABuff::from(v);
                    (a_buff.id, a_buff)
                })
                .collect(),
            abils: c_data
                .abils
                .iter()
                .map(|v| {
                    let a_abil = rc::ad::AAbil::from(v);
                    (a_abil.id, a_abil)
                })
                .collect(),
            item_lists: c_data
                .item_lists
                .iter()
                .map(|v| {
                    let a_item_list = rc::ad::AItemList::from(v);
                    (a_item_list.id, a_item_list)
                })
                .collect(),
        }
    }
}
