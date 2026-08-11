use serde::Deserialize;

use crate::phb::data::{Key, KeyMergeOne};

#[derive(Deserialize)]
pub(in crate::phb) struct PItemFighterAbils {
    #[serde(rename = "abilitySlot0")]
    ability_slot0: Option<PItemFighterAbilData>,
    #[serde(rename = "abilitySlot1")]
    ability_slot1: Option<PItemFighterAbilData>,
    #[serde(rename = "abilitySlot2")]
    ability_slot2: Option<PItemFighterAbilData>,
}
impl KeyMergeOne<rc::ed::EItemAbil> for PItemFighterAbils {
    fn key_merge(self, key: Key) -> Vec<rc::ed::EItemAbil> {
        let mut vec = Vec::new();
        for (slot, p_abil_data) in [self.ability_slot0, self.ability_slot1, self.ability_slot2]
            .into_iter()
            .enumerate()
        {
            let Some(p_abil_data) = p_abil_data else {
                continue;
            };
            let (charge_count, charge_rearm_duration) = p_abil_data
                .charges
                .map_or((None, None), |v| (Some(v.charge_count), Some(v.rearm_time_seconds)));
            vec.push(rc::ed::EItemAbil {
                item_id: rc::ed::EItemId::from_i32(key),
                abil_id: rc::ed::EAbilId::from_i32(p_abil_data.ability_id),
                slot: rc::ed::EInt::from_i32(slot.saturating_cast()),
                cooldown: p_abil_data.cooldown_seconds.map(rc::ed::EFloat::from_f64),
                charge_count: charge_count.map(rc::ed::EInt::from_i32),
                charge_rearm_duration: charge_rearm_duration.map(rc::ed::EFloat::from_f64),
            });
        }
        vec
    }
}

#[derive(Deserialize)]
struct PItemFighterAbilData {
    #[serde(rename = "abilityID")]
    ability_id: i32,
    #[serde(rename = "cooldownSeconds")]
    cooldown_seconds: Option<f64>,
    charges: Option<PItemFighterAbilChargeData>,
}

#[derive(Deserialize)]
struct PItemFighterAbilChargeData {
    #[serde(rename = "chargeCount")]
    charge_count: i32,
    #[serde(rename = "rearmTimeSeconds")]
    rearm_time_seconds: f64,
}
