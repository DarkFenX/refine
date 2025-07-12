use itertools::Itertools;

use crate::{ac, ad, ed};

pub(in crate::adg::flow::conv_pre) fn apply_space_comps(e_data: &ed::EData, a_data: &mut ad::AData) {
    for e_space_comp in e_data.space_comps.data.iter() {
        if !a_data.items.contains_key(&e_space_comp.item_id) {
            continue;
        }
        process_buffs(
            &e_space_comp.system_emitter_buffs,
            a_data,
            e_space_comp.item_id,
            ad::AEffectId::ScSystemEmitter(e_space_comp.item_id),
        );
        process_buffs(
            &e_space_comp.proxy_effect_buffs,
            a_data,
            e_space_comp.item_id,
            ad::AEffectId::ScProxyEffect(e_space_comp.item_id),
        );
        process_buffs(
            &e_space_comp.proxy_trigger_buffs,
            a_data,
            e_space_comp.item_id,
            ad::AEffectId::ScProxyTrap(e_space_comp.item_id),
        );
        process_buffs(
            &e_space_comp.ship_link_buffs,
            a_data,
            e_space_comp.item_id,
            ad::AEffectId::ScShipLink(e_space_comp.item_id),
        );
    }
}

fn process_buffs(
    e_sc_buffs: &[ed::EItemSpaceCompBuff],
    a_data: &mut ad::AData,
    item_id: ad::AItemId,
    effect_id: ad::AEffectId,
) {
    let valid_buffs = e_sc_buffs
        .iter()
        .filter(|v| a_data.buffs.contains_key(&v.id))
        .collect_vec();
    if valid_buffs.is_empty() {
        return;
    }
    let buff_info = ad::AEffectBuffInfo {
        source: ad::AEffectBuffSrc::Customized(
            valid_buffs
                .iter()
                .map(|v| ad::AEffectBuffSrcCustom::HardcodedVal(v.id, ad::AAttrVal::from(v.value)))
                .collect(),
        ),
        scope: ad::AEffectBuffScope::Ships,
    };
    let effect = ad::AEffect {
        id: effect_id,
        category: ac::effcats::ACTIVE,
        state: ad::AState::Offline,
        buff: Some(buff_info),
        ..
    };
    a_data.effects.insert(effect_id, effect);
    a_data
        .items
        .get_mut(&item_id)
        .unwrap()
        .effect_datas
        .insert(effect_id, ad::AItemEffectData::default());
}
