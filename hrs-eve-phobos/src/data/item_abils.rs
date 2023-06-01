use crate::fsd::FsdMerge;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ItemFighterAbils {
    #[serde(rename = "abilitySlot0")]
    pub(crate) abil0: Option<ItemFighterAbilData>,
    #[serde(rename = "abilitySlot1")]
    pub(crate) abil1: Option<ItemFighterAbilData>,
    #[serde(rename = "abilitySlot2")]
    pub(crate) abil2: Option<ItemFighterAbilData>,
}
impl FsdMerge<rc::edt::ItemFighterAbil> for ItemFighterAbils {
    fn fsd_merge(self, id: rc::ReeInt) -> Vec<rc::edt::ItemFighterAbil> {
        let mut vec = Vec::new();
        for abil_data in vec![self.abil0, self.abil1, self.abil2].into_iter() {
            if let Some(abil_data) = abil_data {
                let (charge_count, charge_rearm_time) = abil_data
                    .charges
                    .map_or((None, None), |v| (Some(v.count), Some(v.rearm_time)));
                vec.push(rc::edt::ItemFighterAbil::new(
                    id,
                    abil_data.abil_id,
                    abil_data.cooldown,
                    charge_count,
                    charge_rearm_time,
                ));
            }
        }
        vec
    }
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ItemFighterAbilData {
    #[serde(rename = "abilityID")]
    pub(crate) abil_id: rc::ReeInt,
    #[serde(rename = "cooldownSeconds")]
    pub(crate) cooldown: Option<rc::ReeFloat>,
    pub(crate) charges: Option<ItemFighterAbilChargeData>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ItemFighterAbilChargeData {
    #[serde(rename = "chargeCount")]
    pub(crate) count: rc::ReeInt,
    #[serde(rename = "rearmTimeSeconds")]
    pub(crate) rearm_time: rc::ReeFloat,
}
