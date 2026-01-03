use itertools::Itertools;

use crate::{
    ad::{
        AItem, AItemEffectData, AItemId, ASkillLevel, AState,
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
            id: e_item.id.into(),
            grp_id: e_item.group_id.into(),
            cat_id: cat_eid.into(),
            attrs: RMap::new(),
            effect_datas: RMap::new(),
            defeff_id: defeff_eid.map(Into::into),
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
            .get_mut(&e_item_attr.item_id.into())
            .and_then(|v| v.attrs.insert(e_item_attr.attr_id.into(), e_item_attr.value.into()));
    }
    // Item effects & extended effect data from abilities
    for e_item_effect in e_data.item_effects.data.iter() {
        if let Some(a_item) = a_items.get_mut(&e_item_effect.item_id.into()) {
            a_item
                .effect_datas
                .insert(e_item_effect.effect_id.into(), AItemEffectData::default());
        }
    }
    for e_item_abil in e_data.item_abils.data.iter() {
        match a_items.get_mut(&e_item_abil.item_id.into()) {
            None => continue,
            Some(a_item) => match get_abil_effect(e_item_abil.abil_id) {
                None => continue,
                Some(effect_eid) => match a_item.effect_datas.get_mut(&effect_eid.into()) {
                    None => continue,
                    Some(a_item_eff_data) => {
                        a_item_eff_data.cooldown = e_item_abil.cooldown.map(Into::into);
                        a_item_eff_data.charge_count = e_item_abil.charge_count.map(Into::into);
                        a_item_eff_data.charge_reload_time = e_item_abil.charge_rearm_time.map(Into::into);
                    }
                },
            },
        }
    }
    // Item abilities
    for e_item_abil in e_data
        .item_abils
        .data
        .iter()
        .sorted_unstable_by_key(|v| v.slot.into_inner())
    {
        if let Some(a_item) = a_items.get_mut(&e_item_abil.item_id.into()) {
            a_item.abil_ids.push(e_item_abil.abil_id.into());
        }
    }

    // Item skill requirements
    for e_item_srq in e_data.item_srqs.data.iter() {
        a_items.get_mut(&e_item_srq.item_id.into()).and_then(|v| {
            v.srqs.insert(
                e_item_srq.skill_id.into(),
                ASkillLevel::new_clamped_i32(e_item_srq.level.into_inner()),
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
