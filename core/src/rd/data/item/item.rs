use crate::{
    ac,
    ad::{
        AAbilId, AAttrId, AAttrVal, AEffectId, AItem, AItemCatId, AItemGrpId, AItemId, AItemListId, ASkillLevel, AState,
    },
    rd::{RAttrConsts, RAttrId, REffectConsts, REffectId, RItemAXt, RItemEffectData, RItemListId, RShipKind},
    util::{GetId, RMap},
};

// Represents an item (or item type, according to EVE terminology).
//
// An item carries alot of info needed to calculate fit attributes, for example base attribute
// values.
pub(crate) struct RItem {
    pub(crate) id: AItemId,
    pub(crate) grp_id: AItemGrpId,
    pub(crate) cat_id: AItemCatId,
    pub(crate) abil_ids: Vec<AAbilId>,
    pub(crate) srqs: RMap<AItemId, ASkillLevel>,
    pub(crate) max_state: AState,
    pub(crate) val_fitted_group_id: Option<AItemGrpId>,
    pub(crate) val_online_group_id: Option<AItemGrpId>,
    pub(crate) val_active_group_id: Option<AItemGrpId>,
    pub(crate) is_ice_harvester: bool,
    pub(crate) disallowed_in_wspace: bool,
    // Fields which depend on slab keys
    pub(crate) attrs: RMap<RAttrId, AAttrVal>,
    pub(crate) effect_datas: RMap<REffectId, RItemEffectData>,
    pub(crate) defeff_key: Option<REffectId>,
    pub(crate) proj_buff_item_list_keys: Vec<RItemListId>,
    pub(crate) fleet_buff_item_list_keys: Vec<RItemListId>,
    pub(crate) cap_use_attr_keys: Vec<RAttrId>,
    pub(crate) ship_kind: Option<RShipKind>,
    pub(crate) has_online_effect: bool,
    pub(crate) takes_turret_hardpoint: bool,
    pub(crate) takes_launcher_hardpoint: bool,
    pub(crate) axt: RItemAXt,
}
impl RItem {
    pub(in crate::rd) fn from_a_item(a_item: &AItem) -> Self {
        Self {
            id: a_item.id,
            grp_id: a_item.grp_id,
            cat_id: a_item.cat_id,
            abil_ids: a_item.abil_ids.clone(),
            srqs: a_item.srqs.clone(),
            max_state: a_item.max_state,
            val_fitted_group_id: a_item.val_fitted_group_id,
            val_online_group_id: a_item.val_online_group_id,
            val_active_group_id: a_item.val_active_group_id,
            is_ice_harvester: a_item.is_ice_harvester,
            disallowed_in_wspace: a_item.disallowed_in_wspace,
            // Fields which depend on slab keys
            attrs: Default::default(),
            effect_datas: Default::default(),
            defeff_key: Default::default(),
            proj_buff_item_list_keys: Default::default(),
            fleet_buff_item_list_keys: Default::default(),
            cap_use_attr_keys: Default::default(),
            ship_kind: Default::default(),
            has_online_effect: Default::default(),
            takes_turret_hardpoint: Default::default(),
            takes_launcher_hardpoint: Default::default(),
            axt: Default::default(),
        }
    }
    pub(in crate::rd) fn fill_key_dependents(
        &mut self,
        a_items: &RMap<AItemId, AItem>,
        item_list_id_key_map: &RMap<AItemListId, RItemListId>,
        attr_id_key_map: &RMap<AAttrId, RAttrId>,
        effect_id_key_map: &RMap<AEffectId, REffectId>,
        attr_consts: &RAttrConsts,
        effect_consts: &REffectConsts,
    ) {
        let a_item = a_items.get(&self.id).unwrap();
        for (a_attr_id, attr_value) in a_item.attrs.iter() {
            if let Some(&attr_key) = attr_id_key_map.get(a_attr_id) {
                self.attrs.insert(attr_key, *attr_value);
            }
        }
        for (a_effect_id, a_effect_data) in a_item.effect_datas.iter() {
            if let Some(&effect_key) = effect_id_key_map.get(a_effect_id) {
                let r_effect_data = RItemEffectData::from_a_effect_data(a_effect_data, item_list_id_key_map);
                self.effect_datas.insert(effect_key, r_effect_data);
            }
        }
        self.defeff_key = a_item.defeff_id.and_then(|v| effect_id_key_map.get(&v).copied());
        self.proj_buff_item_list_keys.extend(
            a_item
                .proj_buff_item_list_ids
                .iter()
                .filter_map(|v| item_list_id_key_map.get(v).copied()),
        );
        self.fleet_buff_item_list_keys.extend(
            a_item
                .fleet_buff_item_list_ids
                .iter()
                .filter_map(|v| item_list_id_key_map.get(v).copied()),
        );
        self.cap_use_attr_keys.extend(
            a_item
                .cap_use_attr_ids
                .iter()
                .filter_map(|v| attr_id_key_map.get(v).copied()),
        );
        self.ship_kind = get_ship_kind(self.cat_id, &self.srqs);
        self.has_online_effect = has_online_effect(&self.effect_datas, effect_id_key_map);
        self.takes_turret_hardpoint = has_turret_effect(&self.effect_datas, effect_id_key_map);
        self.takes_launcher_hardpoint = has_launcher_effect(&self.effect_datas, effect_id_key_map);
        self.axt.fill(
            self.id,
            self.grp_id,
            self.cat_id,
            &self.attrs,
            &self.effect_datas,
            attr_id_key_map,
            attr_consts,
            effect_consts,
        );
    }
}
impl GetId<AItemId> for RItem {
    fn get_id(&self) -> AItemId {
        self.id
    }
}

fn has_online_effect(
    item_effects: &RMap<REffectId, RItemEffectData>,
    effect_id_key_map: &RMap<AEffectId, REffectId>,
) -> bool {
    has_effect(item_effects, effect_id_key_map, &ac::effects::ONLINE)
}
fn has_turret_effect(
    item_effects: &RMap<REffectId, RItemEffectData>,
    effect_id_key_map: &RMap<AEffectId, REffectId>,
) -> bool {
    has_effect(item_effects, effect_id_key_map, &ac::effects::TURRET_FITTED)
}
fn has_launcher_effect(
    item_effects: &RMap<REffectId, RItemEffectData>,
    effect_id_key_map: &RMap<AEffectId, REffectId>,
) -> bool {
    has_effect(item_effects, effect_id_key_map, &ac::effects::LAUNCHER_FITTED)
}
fn has_effect(
    item_effects: &RMap<REffectId, RItemEffectData>,
    effect_id_key_map: &RMap<AEffectId, REffectId>,
    effect_id: &AEffectId,
) -> bool {
    let effect_key = match effect_id_key_map.get(effect_id) {
        Some(effect_key) => effect_key,
        None => return false,
    };
    item_effects.contains_key(effect_key)
}

fn get_ship_kind(item_cat_id: AItemCatId, item_srqs: &RMap<AItemId, ASkillLevel>) -> Option<RShipKind> {
    match item_cat_id {
        ac::itemcats::SHIP => match item_srqs.contains_key(&ac::items::CAPITAL_SHIPS) {
            true => Some(RShipKind::CapitalShip),
            false => Some(RShipKind::Ship),
        },
        ac::itemcats::STRUCTURE => Some(RShipKind::Structure),
        _ => None,
    }
}
