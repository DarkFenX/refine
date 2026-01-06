use itertools::Itertools;

use crate::{
    ad::{
        ABuffId, AData, AEffect, AEffectBuff, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope,
        AEffectBuffStrength, AEffectCatId, AEffectId, AItemEffectData, AItemId, AItemListId, AState, AValue,
    },
    ed::{EData, EItemSpaceCompBuffData},
};

pub(in crate::ad::generator::flow::s6_conv_pre) fn apply_space_comps(e_data: &EData, a_data: &mut AData) {
    for e_space_comp in e_data.space_comps.data.iter() {
        let item_aid = AItemId::from_eid(e_space_comp.item_id);
        if !a_data.items.contains_key(&item_aid) {
            continue;
        }
        process_buffs(
            &e_space_comp.system_wide_buffs,
            a_data,
            item_aid,
            AEffectId::ScSystemWide(item_aid),
        );
        process_buffs(
            &e_space_comp.system_emitter_buffs,
            a_data,
            item_aid,
            AEffectId::ScSystemEmitter(item_aid),
        );
        process_buffs(
            &e_space_comp.proxy_effect_buffs,
            a_data,
            item_aid,
            AEffectId::ScProxyEffect(item_aid),
        );
        process_buffs(
            &e_space_comp.proxy_trigger_buffs,
            a_data,
            item_aid,
            AEffectId::ScProxyTrap(item_aid),
        );
        process_buffs(
            &e_space_comp.ship_link_buffs,
            a_data,
            item_aid,
            AEffectId::ScShipLink(item_aid),
        );
    }
}

fn process_buffs(
    e_sc_buff_data: &Option<EItemSpaceCompBuffData>,
    a_data: &mut AData,
    item_aid: AItemId,
    effect_aid: AEffectId,
) {
    let e_sc_buff_data = match e_sc_buff_data {
        Some(e_sc_buff_data) => e_sc_buff_data,
        None => return,
    };
    let valid_buffs = e_sc_buff_data
        .buffs
        .iter()
        .filter(|e_entry| a_data.buffs.contains_key(&ABuffId::from_eid(e_entry.id)))
        .collect_vec();
    if valid_buffs.is_empty() {
        return;
    }
    let item_list_aid = match e_sc_buff_data.item_list_filter {
        Some(item_list_eid) => AItemListId::from_eid(item_list_eid),
        None => AItemListId::SHIPS,
    };
    let buff = AEffectBuff {
        full: valid_buffs
            .iter()
            .map(|v| AEffectBuffFull {
                buff_id: ABuffId::from_eid(v.id),
                strength: AEffectBuffStrength::Hardcoded(AValue::from_efloat(v.value)),
                duration: AEffectBuffDuration::None,
                scope: AEffectBuffScope::Projected(item_list_aid),
            })
            .collect(),
        ..
    };
    let effect = AEffect {
        id: effect_aid,
        category: AEffectCatId::ACTIVE,
        state: AState::Offline,
        buff: Some(buff),
        ..
    };
    a_data.effects.insert(effect_aid, effect);
    a_data
        .items
        .get_mut(&item_aid)
        .unwrap()
        .effect_datas
        .insert(effect_aid, AItemEffectData::default());
}
