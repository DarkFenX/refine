use itertools::Itertools;

use crate::{
    ac,
    ad::{AEffectId, AItem, AItemEffectData, AItemId, AItemList, AItemListId, ASkillLevel, AState},
    adg::{GSupport, get_abil_effect},
    def::OF,
    ed::{EData, EEffectId, EItemId},
    util::RMap,
};

pub(in crate::adg::flow::conv_pre) fn conv_items(e_data: &EData, g_supp: &GSupport) -> RMap<AItemId, AItem> {
    // Auxiliary maps
    let defeff_map = make_item_defeff_map(e_data);
    let mut a_items = RMap::new();
    for e_item in e_data.items.data.iter() {
        // Item category ID
        let cat_id = match g_supp.grp_cat_map.get(&e_item.group_id) {
            Some(&cid) => cid,
            None => {
                let msg = format!("unable to find category ID for {e_item}");
                tracing::warn!("{msg}");
                continue;
            }
        };
        // Item default effect
        let defeff_id = defeff_map.get(&e_item.id).copied();
        // Item construction
        let a_item = AItem {
            id: e_item.id,
            grp_id: e_item.group_id,
            cat_id,
            attrs: RMap::new(),
            effect_datas: RMap::new(),
            defeff_id: defeff_id.map(AEffectId::Dogma),
            abil_ids: Vec::new(),
            srqs: RMap::new(),
            buff_item_list_ids: Vec::new(),
            disallowed_in_wspace: is_disallowed_in_wspace(&e_item.id, &g_supp.item_lists),
            // Following fields are set to some default values, actual values will be set after
            // customization
            max_state: AState::Offline,
            val_fitted_group_id: None,
            val_online_group_id: None,
            val_active_group_id: None,
        };
        a_items.insert(a_item.id, a_item);
    }
    // Item attributes
    for e_item_attr in e_data.item_attrs.data.iter() {
        a_items
            .get_mut(&e_item_attr.item_id)
            .and_then(|v| v.attrs.insert(e_item_attr.attr_id, OF(e_item_attr.value)));
    }
    // Item effects & extended effect data from abilities
    for e_item_effect in e_data.item_effects.data.iter() {
        if let Some(a_item) = a_items.get_mut(&e_item_effect.item_id) {
            a_item
                .effect_datas
                .insert(AEffectId::Dogma(e_item_effect.effect_id), AItemEffectData::default());
        }
    }
    for e_item_abil in e_data.item_abils.data.iter() {
        match a_items.get_mut(&e_item_abil.item_id) {
            None => continue,
            Some(a_item) => match get_abil_effect(e_item_abil.abil_id) {
                None => continue,
                Some(e_effect_id) => match a_item.effect_datas.get_mut(&AEffectId::Dogma(e_effect_id)) {
                    None => continue,
                    Some(a_item_eff_data) => {
                        a_item_eff_data.cd = e_item_abil.cooldown.map(OF);
                        a_item_eff_data.charge_count = e_item_abil.charge_count;
                        a_item_eff_data.charge_reload_time = e_item_abil.charge_rearm_time.map(OF);
                    }
                },
            },
        }
    }
    // Item abilities
    for e_item_abil in e_data.item_abils.data.iter().sorted_unstable_by_key(|v| v.slot) {
        if let Some(a_item) = a_items.get_mut(&e_item_abil.item_id) {
            a_item.abil_ids.push(e_item_abil.abil_id);
        }
    }

    // Item skill requirements
    for e_item_srq in e_data.item_srqs.data.iter() {
        a_items
            .get_mut(&e_item_srq.item_id)
            .and_then(|v| v.srqs.insert(e_item_srq.skill_id, ASkillLevel::new(e_item_srq.level)));
    }
    a_items
}

fn make_item_defeff_map(e_data: &EData) -> RMap<EItemId, EEffectId> {
    e_data
        .item_effects
        .data
        .iter()
        .filter(|v| v.is_default)
        .map(|v| (v.item_id, v.effect_id))
        .collect()
}

fn is_disallowed_in_wspace(e_item_id: &EItemId, type_lists: &RMap<AItemListId, AItemList>) -> bool {
    let type_list = match type_lists.get(&ac::itemlists::WORMHOLE_JUMP_BLACK_LIST) {
        Some(type_list) => type_list,
        None => return false,
    };
    type_list.item_ids.contains(e_item_id)
}
