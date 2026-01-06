use itertools::Itertools;

use crate::{
    ad::{
        AAbilId, AAttrId, ACount, AEffectId, AItem, AItemCatId, AItemEffectData, AItemGrpId, AItemId, ASkillLevel,
        AState, AValue,
        generator::{GSupport, get_abil_effect},
    },
    ed::{EData, EEffectId, EItemId},
    util::RMap,
};

pub(in crate::ad::generator::flow::s6_conv_pre) fn conv_items(
    e_data: &EData,
    g_supp: &GSupport,
) -> RMap<AItemId, AItem> {
    // Auxiliary maps
    let defeff_map = make_item_defeff_map(e_data);
    let mut a_items = RMap::new();
    for e_item in e_data.items.data.iter() {
        // Item category ID
        let cat_eid = match g_supp.grp_cat_map.get(&e_item.group_id) {
            Some(&cat_eid) => cat_eid,
            None => {
                let msg = format!("unable to find category ID for {e_item}");
                tracing::warn!("{msg}");
                continue;
            }
        };
        // Item default effect
        let defeff_eid = defeff_map.get(&e_item.id).copied();
        // Item construction
        let a_item = AItem {
            id: AItemId::from_eid(e_item.id),
            grp_id: AItemGrpId::from_eid(e_item.group_id),
            cat_id: AItemCatId::from_eid(cat_eid),
            attrs: RMap::new(),
            effect_datas: RMap::new(),
            defeff_id: defeff_eid.map(AEffectId::from_eid),
            abil_ids: Vec::new(),
            srqs: RMap::new(),
            // Following fields are set to some default values, actual values will be set after
            // customization
            max_state: AState::Offline,
            proj_buff_item_list_ids: Vec::new(),
            fleet_buff_item_list_ids: Vec::new(),
            val_fitted_group_id: None,
            val_online_group_id: None,
            val_active_group_id: None,
            cap_use_attr_ids: Vec::new(),
            is_ice_harvester: false,
            disallowed_in_wspace: false,
        };
        a_items.insert(a_item.id, a_item);
    }
    // Item attributes
    for e_item_attr in e_data.item_attrs.data.iter() {
        a_items
            .get_mut(&AItemId::from_eid(e_item_attr.item_id))
            .and_then(|a_item| {
                a_item.attrs.insert(
                    AAttrId::from_eid(e_item_attr.attr_id),
                    AValue::from_efloat(e_item_attr.value),
                )
            });
    }
    // Item effects & extended effect data from abilities
    for e_item_effect in e_data.item_effects.data.iter() {
        if let Some(a_item) = a_items.get_mut(&AItemId::from_eid(e_item_effect.item_id)) {
            a_item
                .effect_datas
                .insert(AEffectId::from_eid(e_item_effect.effect_id), AItemEffectData::default());
        }
    }
    for e_item_abil in e_data.item_abils.data.iter() {
        let a_item = match a_items.get_mut(&AItemId::from_eid(e_item_abil.item_id)) {
            Some(a_item) => a_item,
            None => continue,
        };
        let effect_eid = match get_abil_effect(e_item_abil.abil_id) {
            Some(effect_eid) => effect_eid,
            None => continue,
        };
        let a_item_eff_data = match a_item.effect_datas.get_mut(&AEffectId::from_eid(effect_eid)) {
            Some(a_item_eff_data) => a_item_eff_data,
            None => continue,
        };
        a_item_eff_data.cooldown = e_item_abil.cooldown.map(AValue::from_efloat);
        a_item_eff_data.charge_count = e_item_abil.charge_count.map(ACount::from_eint_clamped);
        a_item_eff_data.charge_reload_time = e_item_abil.charge_rearm_time.map(AValue::from_efloat);
    }
    // Item abilities
    for e_item_abil in e_data
        .item_abils
        .data
        .iter()
        .sorted_unstable_by_key(|e_item_abil| e_item_abil.slot.into_i32())
    {
        if let Some(a_item) = a_items.get_mut(&AItemId::from_eid(e_item_abil.item_id)) {
            a_item.abil_ids.push(AAbilId::from_eid(e_item_abil.abil_id));
        }
    }

    // Item skill requirements
    for e_item_srq in e_data.item_srqs.data.iter() {
        a_items
            .get_mut(&AItemId::from_eid(e_item_srq.item_id))
            .and_then(|a_item| {
                a_item.srqs.insert(
                    AItemId::from_eid(e_item_srq.skill_id),
                    ASkillLevel::from_i32_clamped(e_item_srq.level.into_i32()),
                )
            });
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
