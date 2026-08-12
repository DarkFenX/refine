use serde::Deserialize;

use crate::sde::data::ExtractOne;

#[derive(Deserialize)]
pub(in crate::sde) struct SItemAbils {
    #[serde(rename = "_key")]
    item_id: i32,
    #[serde(rename = "abilitySlot0")]
    ability_slot0: Option<SItemAbilData>,
    #[serde(rename = "abilitySlot1")]
    ability_slot1: Option<SItemAbilData>,
    #[serde(rename = "abilitySlot2")]
    ability_slot2: Option<SItemAbilData>,
}
impl ExtractOne<rc::ed::EItemAbil> for SItemAbils {
    fn extract(self) -> Vec<rc::ed::EItemAbil> {
        [
            (0, self.ability_slot0),
            (1, self.ability_slot1),
            (2, self.ability_slot2),
        ]
        .into_iter()
        .filter_map(|(slot, p_abil_data)| {
            let p_abil_data = p_abil_data?;
            Some(rc::ed::EItemAbil {
                item_id: rc::ed::EItemId::from_i32(self.item_id),
                abil_id: rc::ed::EAbilId::from_i32(p_abil_data.ability_id),
                slot: rc::ed::EInt::from_i32(slot),
                cooldown: p_abil_data.cooldown_seconds.map(rc::ed::EFloat::from_f64),
                charge_count: p_abil_data
                    .charges
                    .as_ref()
                    .map(|v| rc::ed::EInt::from_i32(v.charge_count)),
                charge_rearm_duration: p_abil_data
                    .charges
                    .as_ref()
                    .map(|v| rc::ed::EFloat::from_f64(v.rearm_time_seconds)),
            })
        })
        .collect()
    }
}

#[derive(Deserialize)]
struct SItemAbilData {
    #[serde(rename = "abilityID")]
    ability_id: i32,
    #[serde(rename = "cooldownSeconds")]
    cooldown_seconds: Option<f64>,
    charges: Option<SItemAbilChargeData>,
}

#[derive(Deserialize)]
struct SItemAbilChargeData {
    #[serde(rename = "chargeCount")]
    charge_count: i32,
    #[serde(rename = "rearmTimeSeconds")]
    rearm_time_seconds: f64,
}
