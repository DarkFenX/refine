use itertools::Itertools;

use crate::{
    ac,
    ad::{
        AAttrVal, AData, AEffect, AEffectBuffInfo, AEffectBuffScope, AEffectBuffSrc, AEffectBuffSrcCustom, AEffectId,
        AItemEffectData, AItemId, AItemListId, AState,
    },
    ed::{EData, EItemSpaceCompBuffData},
};

pub(in crate::adg::flow::conv_pre) fn apply_space_comps(e_data: &EData, a_data: &mut AData) {
    for e_space_comp in e_data.space_comps.data.iter() {
        if !a_data.items.contains_key(&e_space_comp.item_id) {
            continue;
        }
        process_buffs(
            &e_space_comp.system_wide_buffs,
            a_data,
            e_space_comp.item_id,
            AEffectId::ScSystemWide(e_space_comp.item_id),
        );
        process_buffs(
            &e_space_comp.system_emitter_buffs,
            a_data,
            e_space_comp.item_id,
            AEffectId::ScSystemEmitter(e_space_comp.item_id),
        );
        process_buffs(
            &e_space_comp.proxy_effect_buffs,
            a_data,
            e_space_comp.item_id,
            AEffectId::ScProxyEffect(e_space_comp.item_id),
        );
        process_buffs(
            &e_space_comp.proxy_trigger_buffs,
            a_data,
            e_space_comp.item_id,
            AEffectId::ScProxyTrap(e_space_comp.item_id),
        );
        process_buffs(
            &e_space_comp.ship_link_buffs,
            a_data,
            e_space_comp.item_id,
            AEffectId::ScShipLink(e_space_comp.item_id),
        );
    }
}

fn process_buffs(
    e_sc_buff_data: &Option<EItemSpaceCompBuffData>,
    a_data: &mut AData,
    item_id: AItemId,
    effect_id: AEffectId,
) {
    let e_sc_buff_data = match e_sc_buff_data {
        Some(e_sc_buff_data) => e_sc_buff_data,
        None => return,
    };
    let valid_buffs = e_sc_buff_data
        .buffs
        .iter()
        .filter(|v| a_data.buffs.contains_key(&v.id))
        .collect_vec();
    if valid_buffs.is_empty() {
        return;
    }
    let buff_info = AEffectBuffInfo {
        source: AEffectBuffSrc::Customized(
            valid_buffs
                .iter()
                .map(|v| AEffectBuffSrcCustom::HardcodedVal(v.id, AAttrVal::from(v.value)))
                .collect(),
        ),
        scope: AEffectBuffScope {
            item_list_id: match e_sc_buff_data.item_list_filter {
                Some(e_item_list_id) => AItemListId::Eve(e_item_list_id),
                None => ac::itemlists::SHIPS,
            },
            ..
        },
    };
    let effect = AEffect {
        id: effect_id,
        category: ac::effcats::ACTIVE,
        state: AState::Offline,
        buff_info: Some(buff_info),
        ..
    };
    a_data.effects.insert(effect_id, effect);
    a_data
        .items
        .get_mut(&item_id)
        .unwrap()
        .effect_datas
        .insert(effect_id, AItemEffectData::default());
}
