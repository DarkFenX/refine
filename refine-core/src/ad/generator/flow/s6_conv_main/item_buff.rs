use itertools::Itertools;

use crate::{
    ad::{
        ABuffId, AData, ADataGenerator, AEffect, AEffectBuff, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope,
        AEffectCatId, AEffectId, AEffectModStrength, AItemEffect, AItemId, AItemListId, AState, AValue,
    },
    ed::EItemBuffData,
};

impl ADataGenerator {
    pub(super) fn apply_item_buffs(&mut self) {
        for e_item_buff in self.e_data.item_buffs.data.iter() {
            let item_aid = AItemId::from_eid(e_item_buff.item_id);
            if !self.a_data.items.data.contains_key(&item_aid) {
                continue;
            }
            process_buffs(
                &e_item_buff.system_wide_buffs,
                &mut self.a_data,
                item_aid,
                AEffectId::SystemWide(item_aid),
            );
            process_buffs(
                &e_item_buff.system_emitter_buffs,
                &mut self.a_data,
                item_aid,
                AEffectId::SystemEmitter(item_aid),
            );
            process_buffs(
                &e_item_buff.proxy_effect_buffs,
                &mut self.a_data,
                item_aid,
                AEffectId::ProxyEffect(item_aid),
            );
            process_buffs(
                &e_item_buff.proxy_trigger_buffs,
                &mut self.a_data,
                item_aid,
                AEffectId::ProxyTrap(item_aid),
            );
            process_buffs(
                &e_item_buff.ship_link_buffs,
                &mut self.a_data,
                item_aid,
                AEffectId::ShipLink(item_aid),
            );
        }
    }
}

fn process_buffs(e_sc_buff_data: &Option<EItemBuffData>, a_data: &mut AData, item_aid: AItemId, effect_aid: AEffectId) {
    let Some(e_sc_buff_data) = e_sc_buff_data else {
        return;
    };
    let valid_buffs = e_sc_buff_data
        .buffs
        .iter()
        .filter(|e_entry| a_data.buffs.data.contains_key(&ABuffId::from_eid(e_entry.id)))
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
                strength: AEffectModStrength::Hardcoded(AValue::from_efloat(v.value)),
                duration: AEffectBuffDuration::Effect,
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
    a_data.effects.data.insert(effect_aid, effect);
    a_data
        .items
        .data
        .get_mut(&item_aid)
        .unwrap()
        .effects
        .insert(AItemEffect { id: effect_aid, .. });
}
