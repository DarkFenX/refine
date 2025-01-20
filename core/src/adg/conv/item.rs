use itertools::Itertools;

use crate::{
    ad,
    adg::{GData, GSupport},
    defs::{EEffectId, EItemId},
    ec,
    util::StMap,
};

pub(in crate::adg::conv) fn conv_items(g_data: &GData, g_supp: &GSupport) -> Vec<ad::AItem> {
    // Auxiliary maps
    let defeff_map = g_data
        .item_effects
        .iter()
        .filter(|v| v.is_default)
        .map(|v| (v.item_id, v.effect_id))
        .collect::<StMap<EItemId, EEffectId>>();
    let mut a_item_map = StMap::new();
    for e_item in g_data.items.iter() {
        // Item category ID
        let cat_id = match g_supp.grp_cat_map.get(&e_item.group_id) {
            Some(&cid) => cid,
            None => {
                let msg = format!("unable to find category ID for {}", e_item);
                tracing::warn!("{msg}");
                continue;
            }
        };
        // Item default effect
        let defeff_id = defeff_map.get(&e_item.id).map(|v| *v);
        // Item construction
        let a_item = ad::AItem::new(
            e_item.id,
            e_item.group_id,
            cat_id,
            StMap::new(),
            StMap::new(),
            defeff_id,
            StMap::new(),
        );
        a_item_map.insert(a_item.id, a_item);
    }
    // Item attributes
    for e_item_attr in g_data.item_attrs.iter() {
        a_item_map
            .get_mut(&e_item_attr.item_id)
            .and_then(|v| v.attr_vals.insert(e_item_attr.attr_id, e_item_attr.value));
    }
    // Item effects & extended effect data from abilities
    for e_item_effect in g_data.item_effects.iter() {
        a_item_map.get_mut(&e_item_effect.item_id).and_then(|v| {
            v.effect_datas
                .insert(e_item_effect.effect_id, ad::AItemEffectData::new(None, None, None))
        });
    }
    for e_item_abil in g_data.item_abils.iter() {
        match a_item_map.get_mut(&e_item_abil.item_id) {
            None => continue,
            Some(a_item) => match ec::extras::get_abil_effect(e_item_abil.abil_id) {
                None => continue,
                Some(effect_id) => match a_item.effect_datas.get_mut(&effect_id) {
                    None => continue,
                    Some(a_item_eff_data) => {
                        a_item_eff_data.cd = e_item_abil.cooldown;
                        a_item_eff_data.charge_amount = e_item_abil.charge_count;
                        a_item_eff_data.charge_reload_time = e_item_abil.charge_rearm_time;
                    }
                },
            },
        }
    }
    // Item skill requirements
    for e_item_srq in g_data.item_srqs.iter() {
        a_item_map
            .get_mut(&e_item_srq.item_id)
            .and_then(|v| v.srqs.insert(e_item_srq.skill_id, e_item_srq.level));
    }
    // Item extra data
    let mut a_items = Vec::new();
    for mut a_item in a_item_map.into_iter().map(|(_, v)| v).sorted_by_key(|v| v.id) {
        a_item
            .extras
            .update(a_item.grp_id, a_item.cat_id, &a_item.attr_vals, &a_item.effect_datas);
        a_items.push(a_item);
    }
    a_items
}
