use crate::defines::{ReeFloat, ReeInt};
use crate::dh;

use super::super::fsd::FsdMerge;

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct ItemFighterAbils {
    #[serde(rename = "abilitySlot0")]
    pub(in super::super) abil0: Option<ItemFighterAbilData>,
    #[serde(rename = "abilitySlot1")]
    pub(in super::super) abil1: Option<ItemFighterAbilData>,
    #[serde(rename = "abilitySlot2")]
    pub(in super::super) abil2: Option<ItemFighterAbilData>,
}
impl FsdMerge<dh::ItemFighterAbil> for ItemFighterAbils {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::ItemFighterAbil> {
        let mut vec = Vec::new();
        for abil_data in vec![self.abil0, self.abil1, self.abil2].into_iter() {
            if let Some(abil_data) = abil_data {
                let (charge_count, charge_rearm_time) = abil_data
                    .charges
                    .map(|v| (Some(v.count), Some(v.rearm_time)))
                    .unwrap_or_default();
                vec.push(dh::ItemFighterAbil::new(
                    id,
                    abil_data.abil_id,
                    abil_data.cooldown,
                    charge_count,
                    charge_rearm_time,
                ))
            }
        }
        vec
    }
}

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct ItemFighterAbilData {
    #[serde(rename = "abilityID")]
    pub(in super::super) abil_id: ReeInt,
    #[serde(rename = "cooldownSeconds")]
    pub(in super::super) cooldown: Option<ReeFloat>,
    pub(in super::super) charges: Option<ItemFighterAbilChargeData>,
}

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct ItemFighterAbilChargeData {
    #[serde(rename = "chargeCount")]
    pub(in super::super) count: ReeInt,
    #[serde(rename = "rearmTimeSeconds")]
    pub(in super::super) rearm_time: ReeFloat,
}
