use std::sync::Arc;

use crate::{
    ad::{AAbilId, AAttrId, ABuffId, AData, AEffectId, AItemId, AItemListId},
    rd::{
        RAbil, RAttr, RAttrConsts, RAttrId, RBuff, RBuffId, REffect, REffectConsts, REffectId, RItem, RItemList,
        RItemListId, RMuta,
    },
    util::{PSlab, RMap},
};

pub(crate) type RcData = Arc<RData>;
pub(crate) type RcItem = Arc<RItem>;
pub(crate) type RcEffect = Arc<REffect>;
pub(crate) type RcMuta = Arc<RMuta>;

pub(crate) struct RData {
    items: RMap<AItemId, RcItem>,
    item_lists: PSlab<RItemListId, RItemList>,
    item_list_aid_rid_map: RMap<AItemListId, RItemListId>,
    attrs: PSlab<RAttrId, RAttr>,
    attr_aid_rid_map: RMap<AAttrId, RAttrId>,
    attr_consts: RAttrConsts,
    effects: PSlab<REffectId, RcEffect>,
    effect_aid_rid_map: RMap<AEffectId, REffectId>,
    effect_consts: REffectConsts,
    buffs: PSlab<RBuffId, RBuff>,
    buff_aid_rid_map: RMap<ABuffId, RBuffId>,
    mutas: RMap<AItemId, RcMuta>,
    abils: RMap<AAbilId, RAbil>,
    // Extra data stored directly on RData for ease of access / optimization purposes
    online_effect: Option<RcEffect>,
    rah_duration_attr_rid: Option<RAttrId>,
}
impl RData {
    // Item methods
    pub(crate) fn get_item_by_aid(&self, item_aid: &AItemId) -> Option<&RcItem> {
        self.items.get(item_aid)
    }
    // Item list methods
    pub(crate) fn get_item_list_by_rid(&self, item_list_rid: RItemListId) -> &RItemList {
        self.item_lists.get(item_list_rid).unwrap()
    }
    pub(crate) fn get_item_list_rid_by_aid(&self, item_list_aid: &AItemListId) -> Option<RItemListId> {
        self.item_list_aid_rid_map.get(item_list_aid).copied()
    }
    // Attr methods
    pub(crate) fn get_attr_by_rid(&self, attr_rid: RAttrId) -> &RAttr {
        self.attrs.get(attr_rid).unwrap()
    }
    pub(crate) fn get_attr_rid_by_aid(&self, attr_aid: &AAttrId) -> Option<RAttrId> {
        self.attr_aid_rid_map.get(attr_aid).copied()
    }
    pub(crate) fn get_attr_aid_rid_map(&self) -> &RMap<AAttrId, RAttrId> {
        &self.attr_aid_rid_map
    }
    pub(crate) fn get_attr_consts(&self) -> &RAttrConsts {
        &self.attr_consts
    }
    // Attr methods
    pub(crate) fn get_effect_by_rid(&self, effect_rid: REffectId) -> &RcEffect {
        self.effects.get(effect_rid).unwrap()
    }
    pub(crate) fn get_effect_rid_by_aid(&self, effect_aid: &AEffectId) -> Option<REffectId> {
        self.effect_aid_rid_map.get(effect_aid).copied()
    }
    pub(crate) fn get_effect_consts(&self) -> &REffectConsts {
        &self.effect_consts
    }
    // Buff methods
    pub(crate) fn get_buff_by_rid(&self, buff_rid: RBuffId) -> &RBuff {
        self.buffs.get(buff_rid).unwrap()
    }
    pub(crate) fn get_buff_by_aid(&self, buff_aid: &ABuffId) -> Option<&RBuff> {
        let buff_rid = *self.buff_aid_rid_map.get(buff_aid)?;
        Some(self.get_buff_by_rid(buff_rid))
    }
    // Mutator methods
    pub(crate) fn get_mutator_by_aid(&self, item_aid: &AItemId) -> Option<&RcMuta> {
        self.mutas.get(item_aid)
    }
    // Ability methods
    pub(crate) fn get_ability_by_aid(&self, ability_aid: &AAbilId) -> Option<&RAbil> {
        self.abils.get(ability_aid)
    }
    // Misc getters
    pub(crate) fn get_online_effect(&self) -> Option<&RcEffect> {
        self.online_effect.as_ref()
    }
    pub(crate) fn get_rah_duration_attr_rid(&self) -> Option<RAttrId> {
        self.rah_duration_attr_rid
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RData {
    pub(crate) fn from_a_data(a_data: AData) -> Self {
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
        // Item lists
        let mut item_list_aid_rid_map = RMap::with_capacity(a_data.item_lists.data.len());
        let mut item_lists = PSlab::with_capacity(a_data.item_lists.data.len());
        for a_item_list in a_data.item_lists.iter() {
            let entry = item_lists.vacant_entry();
            let item_list_rid = entry.id();
            let r_item_list = RItemList::from_a_item_list(item_list_rid, a_item_list);
            entry.insert(r_item_list);
            item_list_aid_rid_map.insert(a_item_list.id, item_list_rid);
        }
        // Attributes
        let mut attr_aid_rid_map = RMap::with_capacity(a_data.attrs.data.len());
        let mut attrs = PSlab::with_capacity(a_data.attrs.data.len());
        for a_attr in a_data.attrs.iter() {
            let entry = attrs.vacant_entry();
            let attr_rid = entry.id();
            let r_attr = RAttr::from_a_attr(attr_rid, a_attr);
            entry.insert(r_attr);
            attr_aid_rid_map.insert(a_attr.id, attr_rid);
        }
        // Effects
        let mut effect_aid_rid_map = RMap::with_capacity(a_data.effects.data.len());
        let mut effects = PSlab::with_capacity(a_data.effects.data.len());
        for a_effect in a_data.effects.iter() {
            let entry = effects.vacant_entry();
            let effect_rid = entry.id();
            let r_effect = REffect::from_a_effect(effect_rid, a_effect);
            entry.insert(Arc::new(r_effect));
            effect_aid_rid_map.insert(a_effect.id, effect_rid);
        }
        // Buffs
        let mut buff_aid_rid_map = RMap::with_capacity(a_data.buffs.data.len());
        let mut buffs = PSlab::with_capacity(a_data.buffs.data.len());
        for a_buff in a_data.buffs.iter() {
            let entry = buffs.vacant_entry();
            let buff_rid = entry.id();
            let r_buff = RBuff::from_a_buff(a_buff);
            entry.insert(r_buff);
            buff_aid_rid_map.insert(a_buff.id, buff_rid);
        }
        // Create runtime "constants"
        let attr_consts = RAttrConsts::from_id_map(&attr_aid_rid_map);
        let effect_consts = REffectConsts::from_id_map(&effect_aid_rid_map);
        // Fill in data which wasn't filled during instantiation (e.g. depends on slab keys)
        for r_effect in effects.values_mut() {
            Arc::get_mut(r_effect).unwrap().fill_runtime(
                &a_data.effects.data,
                &item_list_aid_rid_map,
                &attr_aid_rid_map,
                &effect_aid_rid_map,
                &buff_aid_rid_map,
            );
        }
        // Item data depends on effect data being filled, so do it after effects
        for r_item in items.values_mut() {
            Arc::get_mut(r_item).unwrap().fill_runtime(
                &a_data.items.data,
                &item_list_aid_rid_map,
                &attr_aid_rid_map,
                &effect_aid_rid_map,
                &attr_consts,
                &effect_consts,
                &effects,
            );
        }
        for r_attr in attrs.values_mut() {
            r_attr.fill_runtime(&a_data.attrs.data, &attr_aid_rid_map);
        }
        for r_buff in buffs.values_mut() {
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
        // Extra data
        let rah_duration_attr_rid = effect_consts
            .adaptive_armor_hardener
            .and_then(|effect_rid| effects.get(effect_rid).unwrap().duration_attr_rid);
        let online_effect = effect_consts
            .online
            .map(|effect_rid| effects.get(effect_rid).unwrap().clone());
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
            rah_duration_attr_rid,
            online_effect,
        }
    }
}
