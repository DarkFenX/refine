use itertools::Itertools;

use crate::{
    ad::{
        AData, AEffect, AEffectBuff, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope, AEffectBuffStrength,
        AEffectCatId, AEffectId, AItemEffectData, AItemId, AItemListId, AState,
    },
    ed::{EData, EItemSpaceCompBuffData},
};

pub(in crate::ad::generator::flow::s6_conv_pre) fn apply_space_comps(e_data: &EData, a_data: &mut AData) {
    for e_space_comp in e_data.space_comps.data.iter() {
        if !a_data.items.contains_key(&e_space_comp.item_id.into()) {
            continue;
        }
        process_buffs(
            &e_space_comp.system_wide_buffs,
            a_data,
            e_space_comp.item_id.into(),
            AEffectId::ScSystemWide(e_space_comp.item_id.into()),
        );
        process_buffs(
            &e_space_comp.system_emitter_buffs,
            a_data,
            e_space_comp.item_id.into(),
            AEffectId::ScSystemEmitter(e_space_comp.item_id.into()),
        );
        process_buffs(
            &e_space_comp.proxy_effect_buffs,
            a_data,
            e_space_comp.item_id.into(),
            AEffectId::ScProxyEffect(e_space_comp.item_id.into()),
        );
        process_buffs(
            &e_space_comp.proxy_trigger_buffs,
            a_data,
            e_space_comp.item_id.into(),
            AEffectId::ScProxyTrap(e_space_comp.item_id.into()),
        );
        process_buffs(
            &e_space_comp.ship_link_buffs,
            a_data,
            e_space_comp.item_id.into(),
            AEffectId::ScShipLink(e_space_comp.item_id.into()),
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
        .filter(|v| a_data.buffs.contains_key(&v.id.into()))
        .collect_vec();
    if valid_buffs.is_empty() {
        return;
    }
    let item_list_aid = match e_sc_buff_data.item_list_filter {
        Some(item_list_eid) => item_list_eid.into(),
        None => AItemListId::SHIPS,
    };
    let buff = AEffectBuff {
        full: valid_buffs
            .iter()
            .map(|v| AEffectBuffFull {
                buff_id: v.id.into(),
                strength: AEffectBuffStrength::Hardcoded(v.value.into()),
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
