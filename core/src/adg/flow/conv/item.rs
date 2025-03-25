use itertools::Itertools;
use ordered_float::OrderedFloat as OF;

use crate::{
    ad,
    adg::{EData, GSupport, get_abil_effect},
    ed,
    util::StMap,
};

pub(in crate::adg::flow::conv) fn conv_items(e_data: &EData, g_supp: &GSupport) -> Vec<ad::AItem> {
    // Auxiliary maps
    let defeff_map = e_data
        .item_effects
        .iter()
        .filter(|v| v.is_default)
        .map(|v| (v.item_id, v.effect_id))
        .collect::<StMap<ed::EItemId, ed::EEffectId>>();
    let mut a_item_map = StMap::new();
    for e_item in e_data.items.iter() {
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
        let defeff_id = defeff_map.get(&e_item.id).copied();
        // Item construction
        let a_item = ad::AItem {
            id: e_item.id,
            grp_id: e_item.group_id,
            cat_id,
            attrs: StMap::new(),
            effect_datas: StMap::new(),
            defeff_id: defeff_id.map(|e_defeff_id| ad::AEffectId::Dogma(e_defeff_id)),
            srqs: StMap::new(),
            extras: ad::AItemExtras::new(),
        };
        a_item_map.insert(a_item.id, a_item);
    }
    // Item attributes
    for e_item_attr in e_data.item_attrs.iter() {
        a_item_map
            .get_mut(&e_item_attr.item_id)
            .and_then(|v| v.attrs.insert(e_item_attr.attr_id, OF(e_item_attr.value)));
    }
    // Item effects & extended effect data from abilities
    for e_item_effect in e_data.item_effects.iter() {
        a_item_map.get_mut(&e_item_effect.item_id).and_then(|v| {
            v.effect_datas.insert(
                ad::AEffectId::Dogma(e_item_effect.effect_id),
                ad::AItemEffectData {
                    cd: None,
                    charge_count: None,
                    charge_reload_time: None,
                },
            )
        });
    }
    for e_item_abil in e_data.item_abils.iter() {
        match a_item_map.get_mut(&e_item_abil.item_id) {
            None => continue,
            Some(a_item) => match get_abil_effect(e_item_abil.abil_id) {
                None => continue,
                Some(e_effect_id) => match a_item.effect_datas.get_mut(&ad::AEffectId::Dogma(e_effect_id)) {
                    None => continue,
                    Some(a_item_eff_data) => {
                        a_item_eff_data.cd = e_item_abil.cooldown.map(|v| OF(v));
                        a_item_eff_data.charge_count = e_item_abil.charge_count;
                        a_item_eff_data.charge_reload_time = e_item_abil.charge_rearm_time.map(|v| OF(v));
                    }
                },
            },
        }
    }
    // Item skill requirements
    for e_item_srq in e_data.item_srqs.iter() {
        a_item_map
            .get_mut(&e_item_srq.item_id)
            .and_then(|v| v.srqs.insert(e_item_srq.skill_id, e_item_srq.level));
    }
    // Item extra data
    let mut a_items = Vec::new();
    for a_item in a_item_map.into_iter().map(|(_, v)| v).sorted_by_key(|v| v.id) {
        a_items.push(a_item);
    }
    a_items
}
