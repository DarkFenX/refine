use std::{
    hash::{BuildHasher, Hash},
    sync::Arc,
};

use slab::Slab;

use crate::{
    ad::{AAbilId, AAttrId, ABuffId, AData, AEffectId, AItemId, AItemListId},
    rd::{
        RAbil, RAttr, RAttrConsts, RAttrKey, RBuff, RBuffKey, REffect, REffectConsts, REffectKey, RItem, RItemList,
        RItemListKey, RMuta, RcAbil, RcAttr, RcBuff, RcEffect, RcItem, RcItemList, RcMuta,
    },
    util::{GetId, Map, RMap},
};

#[derive(Clone)]
pub(in crate::rd) struct RData {
    pub(in crate::rd) items: RMap<AItemId, RcItem>,
    pub(in crate::rd) item_lists: Slab<RcItemList>,
    pub(in crate::rd) item_list_id_key_map: RMap<AItemListId, RItemListKey>,
    pub(in crate::rd) attrs: Slab<RcAttr>,
    pub(in crate::rd) attr_id_key_map: RMap<AAttrId, RAttrKey>,
    pub(in crate::rd) attr_consts: RAttrConsts,
    pub(in crate::rd) effects: Slab<RcEffect>,
    pub(in crate::rd) effect_id_key_map: RMap<AEffectId, REffectKey>,
    pub(in crate::rd) effect_consts: REffectConsts,
    pub(in crate::rd) buffs: Slab<RcBuff>,
    pub(in crate::rd) buff_id_key_map: RMap<ABuffId, RBuffKey>,
    pub(in crate::rd) mutas: RMap<AItemId, RcMuta>,
    pub(in crate::rd) abils: RMap<AAbilId, RcAbil>,
}
impl From<AData> for RData {
    fn from(a_data: AData) -> Self {
        let mut items = move_to_arcmap(a_data.items.values().map(RItem::from_a_item));
        let mut mutas = move_to_arcmap(a_data.mutas.values().map(RMuta::from_a_muta));
        let mut abils = move_to_arcmap(a_data.abils.values().map(RAbil::from_a_abil));
        // Slab item lists
        let mut item_list_id_key_map = RMap::with_capacity(a_data.item_lists.len());
        let mut item_lists = Slab::with_capacity(a_data.item_lists.len());
        for (&a_item_list_id, a_item_list) in a_data.item_lists.iter() {
            let entry = item_lists.vacant_entry();
            let item_list_key = entry.key();
            let r_item_list = RItemList::from_a_item_list(item_list_key, a_item_list);
            entry.insert(Arc::new(r_item_list));
            item_list_id_key_map.insert(a_item_list_id, item_list_key);
        }
        // Slab attributes
        let mut attr_id_key_map = RMap::with_capacity(a_data.attrs.len());
        let mut attrs = Slab::with_capacity(a_data.attrs.len());
        for (&a_attr_id, a_attr) in a_data.attrs.iter() {
            let entry = attrs.vacant_entry();
            let attr_key = entry.key();
            let r_attr = RAttr::from_a_attr(attr_key, a_attr);
            entry.insert(Arc::new(r_attr));
            attr_id_key_map.insert(a_attr_id, attr_key);
        }
        // Slab effects
        let mut effect_id_key_map = RMap::with_capacity(a_data.effects.len());
        let mut effects = Slab::with_capacity(a_data.effects.len());
        for (&a_effect_id, a_effect) in a_data.effects.iter() {
            let entry = effects.vacant_entry();
            let effect_key = entry.key();
            let r_effect = REffect::from_a_effect(effect_key, a_effect);
            entry.insert(Arc::new(r_effect));
            effect_id_key_map.insert(a_effect_id, effect_key);
        }
        // Slab buffs
        let mut buff_id_key_map = RMap::with_capacity(a_data.buffs.len());
        let mut buffs = Slab::with_capacity(a_data.buffs.len());
        for (&a_buff_id, a_buff) in a_data.buffs.iter() {
            let entry = buffs.vacant_entry();
            let buff_key = entry.key();
            let r_buff = RBuff::from_a_buff(buff_key, a_buff);
            entry.insert(Arc::new(r_buff));
            buff_id_key_map.insert(a_buff_id, buff_key);
        }
        // Create runtime "constants"
        let attr_consts = RAttrConsts::new(&attr_id_key_map);
        let effect_consts = REffectConsts::new(&effect_id_key_map);
        // Refresh data which relies on effects' slab keys
        for r_item in items.values_mut() {
            Arc::get_mut(r_item).unwrap().fill_key_dependents(
                &a_data.items,
                &item_list_id_key_map,
                &attr_id_key_map,
                &effect_id_key_map,
                &attr_consts,
                &effect_consts,
            );
        }
        for (_, r_attr) in attrs.iter_mut() {
            Arc::get_mut(r_attr)
                .unwrap()
                .fill_key_dependents(&a_data.attrs, &attr_id_key_map);
        }
        for (_, r_effect) in effects.iter_mut() {
            Arc::get_mut(r_effect).unwrap().fill_key_dependents(
                &a_data.effects,
                &item_list_id_key_map,
                &attr_id_key_map,
                &effect_id_key_map,
                &buff_id_key_map,
            );
        }
        for (_, r_buff) in buffs.iter_mut() {
            Arc::get_mut(r_buff)
                .unwrap()
                .fill_key_dependents(&a_data.buffs, &attr_id_key_map);
        }
        for r_muta in mutas.values_mut() {
            Arc::get_mut(r_muta)
                .unwrap()
                .fill_key_dependents(&a_data.mutas, &attr_id_key_map);
        }
        for r_abil in abils.values_mut() {
            Arc::get_mut(r_abil).unwrap().fill_key_dependents(&effect_id_key_map);
        }
        Self {
            items,
            item_lists,
            item_list_id_key_map,
            attrs,
            attr_id_key_map,
            attr_consts,
            effects,
            effect_id_key_map,
            effect_consts,
            buffs,
            buff_id_key_map,
            mutas,
            abils,
        }
    }
}

fn move_to_arcmap<K, V, H>(entities: impl ExactSizeIterator<Item = V>) -> Map<K, Arc<V>, H>
where
    K: Eq + PartialEq + Hash,
    V: GetId<K>,
    H: BuildHasher + Default,
{
    entities.map(|v| (v.get_id(), Arc::new(v))).collect()
}
