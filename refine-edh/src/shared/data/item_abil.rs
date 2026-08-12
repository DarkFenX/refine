use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct ItemAbilData {
    #[serde(rename = "abilityID")]
    ability_id: i32,
    #[serde(rename = "cooldownSeconds")]
    cooldown_seconds: Option<f64>,
    charges: Option<ItemAbilChargeData>,
}

#[derive(Deserialize)]
struct ItemAbilChargeData {
    #[serde(rename = "chargeCount")]
    charge_count: i32,
    #[serde(rename = "rearmTimeSeconds")]
    rearm_time_seconds: f64,
}

pub(crate) fn into_e_item_abils(item_id: i32, abil_datas: [Option<ItemAbilData>; 3]) -> Vec<rc::ed::EItemAbil> {
    abil_datas
        .into_iter()
        .zip(0..)
        .filter_map(|(abil_data, slot)| {
            let abil_data = abil_data?;
            Some(rc::ed::EItemAbil {
                item_id: rc::ed::EItemId::from_i32(item_id),
                abil_id: rc::ed::EAbilId::from_i32(abil_data.ability_id),
                slot: rc::ed::EInt::from_i32(slot),
                cooldown: abil_data.cooldown_seconds.map(rc::ed::EFloat::from_f64),
                charge_count: abil_data
                    .charges
                    .as_ref()
                    .map(|v| rc::ed::EInt::from_i32(v.charge_count)),
                charge_rearm_duration: abil_data
                    .charges
                    .as_ref()
                    .map(|v| rc::ed::EFloat::from_f64(v.rearm_time_seconds)),
            })
        })
        .collect()
}
