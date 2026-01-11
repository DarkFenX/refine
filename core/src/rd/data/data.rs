use std::sync::Arc;

use slab::Slab;

use crate::{
    ad::{AAbilId, AAttrId, ABuffId, AData, AEffectId, AItemId, AItemListId},
    rd::{
        RAbil, RAttr, RAttrConsts, RAttrId, RBuff, RBuffId, REffect, REffectConsts, REffectId, RItem, RItemList,
        RItemListId, RMuta,
    },
    util::RMap,
};

pub(crate) type RcItem = Arc<RItem>;
pub(crate) type RcEffect = Arc<REffect>;
pub(crate) type RcMuta = Arc<RMuta>;

pub(in crate::rd) struct RData {
    pub(in crate::rd) items: RMap<AItemId, RcItem>,
    pub(in crate::rd) item_lists: Slab<RItemList>,
    pub(in crate::rd) item_list_aid_rid_map: RMap<AItemListId, RItemListId>,
    pub(in crate::rd) attrs: Slab<RAttr>,
    pub(in crate::rd) attr_aid_rid_map: RMap<AAttrId, RAttrId>,
    pub(in crate::rd) attr_consts: RAttrConsts,
    pub(in crate::rd) effects: Slab<RcEffect>,
    pub(in crate::rd) effect_aid_rid_map: RMap<AEffectId, REffectId>,
    pub(in crate::rd) effect_consts: REffectConsts,
    pub(in crate::rd) buffs: Slab<RBuff>,
    pub(in crate::rd) buff_aid_rid_map: RMap<ABuffId, RBuffId>,
    pub(in crate::rd) mutas: RMap<AItemId, RcMuta>,
    pub(in crate::rd) abils: RMap<AAbilId, RAbil>,
}
impl RData {
    pub(in crate::rd) fn from_a_data(a_data: AData) -> Self {
        let mut items: RMap<_, _> = a_data
            .items
            .data
            .values()
            .map(|a_item| (a_item.id, Arc::new(RItem::from_a_item(a_item))))
            .collect();
        let mut mutas: RMap<_, _> = a_data
            .mutas
            .data
            .values()
            .map(|a_muta| (a_muta.id, Arc::new(RMuta::from_a_muta(a_muta))))
            .collect();
        let mut abils: RMap<_, _> = a_data
            .abils
            .data
            .values()
            .map(|a_abil| (a_abil.id, RAbil::from_a_abil(a_abil)))
            .collect();
        // Slab item lists
        let mut item_list_aid_rid_map = RMap::with_capacity(a_data.item_lists.data.len());
        let mut item_lists = Slab::with_capacity(a_data.item_lists.data.len());
        for a_item_list in a_data.item_lists.iter() {
            let entry = item_lists.vacant_entry();
            let item_list_rid = RItemListId::from_usize(entry.key());
            let r_item_list = RItemList::from_a_item_list(item_list_rid, a_item_list);
            entry.insert(r_item_list);
            item_list_aid_rid_map.insert(a_item_list.id, item_list_rid);
        }
        // Slab attributes
        let mut attr_aid_rid_map = RMap::with_capacity(a_data.attrs.data.len());
        let mut attrs = Slab::with_capacity(a_data.attrs.data.len());
        for a_attr in a_data.attrs.iter() {
            let entry = attrs.vacant_entry();
            let attr_rid = RAttrId::from_usize(entry.key());
            let r_attr = RAttr::from_a_attr(attr_rid, a_attr);
            entry.insert(r_attr);
            attr_aid_rid_map.insert(a_attr.id, attr_rid);
        }
        // Slab effects
        let mut effect_aid_rid_map = RMap::with_capacity(a_data.effects.data.len());
        let mut effects = Slab::with_capacity(a_data.effects.data.len());
        for a_effect in a_data.effects.iter() {
            let entry = effects.vacant_entry();
            let effect_rid = REffectId::from_usize(entry.key());
            let r_effect = REffect::from_a_effect(effect_rid, a_effect);
            entry.insert(Arc::new(r_effect));
            effect_aid_rid_map.insert(a_effect.id, effect_rid);
        }
        // Slab buffs
        let mut buff_aid_rid_map = RMap::with_capacity(a_data.buffs.data.len());
        let mut buffs = Slab::with_capacity(a_data.buffs.data.len());
        for a_buff in a_data.buffs.iter() {
            let entry = buffs.vacant_entry();
            let buff_rid = RBuffId::from_usize(entry.key());
            let r_buff = RBuff::from_a_buff(a_buff);
            entry.insert(r_buff);
            buff_aid_rid_map.insert(a_buff.id, buff_rid);
        }
        // Create runtime "constants"
        let attr_consts = RAttrConsts::new(&attr_aid_rid_map);
        let effect_consts = REffectConsts::new(&effect_aid_rid_map);
        // Fill in data which wasn't filled during instantiation (e.g. depends on slab keys)
        for r_item in items.values_mut() {
            Arc::get_mut(r_item).unwrap().fill_runtime(
                &a_data.items.data,
                &item_list_aid_rid_map,
                &attr_aid_rid_map,
                &effect_aid_rid_map,
                &attr_consts,
                &effect_consts,
            );
        }
        for (_, r_attr) in attrs.iter_mut() {
            r_attr.fill_runtime(&a_data.attrs.data, &attr_aid_rid_map);
        }
        for (_, r_effect) in effects.iter_mut() {
            Arc::get_mut(r_effect).unwrap().fill_runtime(
                &a_data.effects.data,
                &item_list_aid_rid_map,
                &attr_aid_rid_map,
                &effect_aid_rid_map,
                &buff_aid_rid_map,
            );
        }
        for (_, r_buff) in buffs.iter_mut() {
            r_buff.fill_runtime(&a_data.buffs.data, &attr_aid_rid_map);
        }
        for r_muta in mutas.values_mut() {
            Arc::get_mut(r_muta)
                .unwrap()
                .fill_runtime(&a_data.mutas.data, &attr_aid_rid_map);
        }
        for r_abil in abils.values_mut() {
            r_abil.fill_runtime(&effect_aid_rid_map);
        }
        Self {
            items,
            item_lists,
            item_list_aid_rid_map,
            attrs,
            attr_aid_rid_map,
            attr_consts,
            effects,
            effect_aid_rid_map,
            effect_consts,
            buffs,
            buff_aid_rid_map,
            mutas,
            abils,
        }
    }
}
