use itertools::Itertools;

use crate::{
    ac,
    ad::{
        AAttrVal, AData, AEffect, AEffectBuffInfo, AEffectBuffScope, AEffectBuffSrc, AEffectBuffSrcCustom, AEffectId,
        AItemEffectData, AItemId, AState,
    },
    ed::{EData, EItemSpaceCompBuff},
};

pub(in crate::adg::flow::conv_pre) fn apply_space_comps(e_data: &EData, a_data: &mut AData) {
    for e_space_comp in e_data.space_comps.data.iter() {
        if !a_data.items.contains_key(&e_space_comp.item_id) {
            continue;
        }
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

fn process_buffs(e_sc_buffs: &[EItemSpaceCompBuff], a_data: &mut AData, item_id: AItemId, effect_id: AEffectId) {
    let valid_buffs = e_sc_buffs
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
        scope: AEffectBuffScope::Ships,
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
