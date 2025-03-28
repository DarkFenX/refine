use itertools::Itertools;

use crate::{ac, ad, adg::EData, ed};

pub(in crate::adg::flow::conv) fn apply_space_comps(e_data: &EData, a_data: &mut ad::AData) {
    for e_space_comp in e_data.space_comps.iter() {
        if !a_data.items.contains_key(&e_space_comp.item_id) {
            continue;
        }
        process_buffs(
            &e_space_comp.system_emitter_buffs,
            a_data,
            e_space_comp.item_id,
            ad::AEffectId::ScSystemEmitter(e_space_comp.item_id),
            ad::AEffectBuffScope::Ships,
        );
        process_buffs(
            &e_space_comp.proxy_effect_buffs,
            a_data,
            e_space_comp.item_id,
            ad::AEffectId::ScProxyEffect(e_space_comp.item_id),
            ad::AEffectBuffScope::Ships,
        );
        process_buffs(
            &e_space_comp.proxy_trigger_buffs,
            a_data,
            e_space_comp.item_id,
            ad::AEffectId::ScProxyTrigger(e_space_comp.item_id),
            ad::AEffectBuffScope::Everything,
        );
        process_buffs(
            &e_space_comp.ship_link_buffs,
            a_data,
            e_space_comp.item_id,
            ad::AEffectId::ScShipLink(e_space_comp.item_id),
            ad::AEffectBuffScope::Ships,
        );
    }
}

fn process_buffs(
    e_sc_buffs: &[ed::EItemSpaceCompBuff],
    a_data: &mut ad::AData,
    item_id: ad::AItemId,
    effect_id: ad::AEffectId,
    scope: ad::AEffectBuffScope,
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
        scope,
    };
    let effect = ad::AEffect {
        id: effect_id,
        category: ac::effcats::ACTIVE,
        state: ad::AState::Offline,
        is_assist: false,
        is_offense: false,
        hisec: None,
        lowsec: None,
        discharge_attr_id: None,
        duration_attr_id: None,
        range_attr_id: None,
        falloff_attr_id: None,
        track_attr_id: None,
        chance_attr_id: None,
        resist_attr_id: None,
        mod_build_status: ad::AEffectModBuildStatus::Success,
        mods: Vec::new(),
        stop_ids: Vec::new(),
        buff: Some(buff_info),
        charge: None,
    };
    a_data.effects.insert(effect_id, effect);
    a_data.items.get_mut(&item_id).unwrap().effect_datas.insert(
        effect_id,
        ad::AItemEffectData {
            cd: None,
            charge_count: None,
            charge_reload_time: None,
        },
    );
}
