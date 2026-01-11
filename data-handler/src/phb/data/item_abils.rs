use serde::Deserialize;

use crate::phb::fsd::{FsdId, FsdMerge};

#[derive(Deserialize)]
pub(in crate::phb) struct PItemFighterAbils {
    #[serde(rename = "abilitySlot0")]
    pub(in crate::phb) abil0: Option<PItemFighterAbilData>,
    #[serde(rename = "abilitySlot1")]
    pub(in crate::phb) abil1: Option<PItemFighterAbilData>,
    #[serde(rename = "abilitySlot2")]
    pub(in crate::phb) abil2: Option<PItemFighterAbilData>,
}
impl FsdMerge<rc::ed::EItemAbil> for PItemFighterAbils {
    fn fsd_merge(self, id: FsdId) -> Vec<rc::ed::EItemAbil> {
        let mut vec = Vec::new();
        for (slot, p_abil_data) in [self.abil0, self.abil1, self.abil2].into_iter().enumerate() {
            let p_abil_data = match p_abil_data {
                Some(p_abil_data) => p_abil_data,
                None => continue,
            };
            let (charge_count, charge_rearm_time) = p_abil_data
                .charges
                .map_or((None, None), |v| (Some(v.count), Some(v.rearm_time)));
            vec.push(rc::ed::EItemAbil {
                item_id: rc::ed::EItemId::from_i32(id),
                abil_id: rc::ed::EAbilId::from_i32(p_abil_data.abil_id),
                slot: rc::ed::EInt::from_i32(slot as i32),
                cooldown: p_abil_data.cooldown.map(rc::ed::EFloat::from_f64),
                charge_count: charge_count.map(rc::ed::EInt::from_i32),
                charge_rearm_time: charge_rearm_time.map(rc::ed::EFloat::from_f64),
            });
        }
        vec
    }
}

#[derive(Deserialize)]
pub(in crate::phb) struct PItemFighterAbilData {
    #[serde(rename = "abilityID")]
    pub(in crate::phb) abil_id: i32,
    #[serde(rename = "cooldownSeconds")]
    pub(in crate::phb) cooldown: Option<f64>,
    pub(in crate::phb) charges: Option<PItemFighterAbilChargeData>,
}

#[derive(Deserialize)]
pub(in crate::phb) struct PItemFighterAbilChargeData {
    #[serde(rename = "chargeCount")]
    pub(in crate::phb) count: i32,
    #[serde(rename = "rearmTimeSeconds")]
    pub(in crate::phb) rearm_time: f64,
}
