use crate::phb::fsd::FsdMerge;

#[derive(Debug, serde::Deserialize)]
pub(in crate::phb) struct PItemFighterAbils {
    #[serde(rename = "abilitySlot0")]
    pub(in crate::phb) abil0: Option<PItemFighterAbilData>,
    #[serde(rename = "abilitySlot1")]
    pub(in crate::phb) abil1: Option<PItemFighterAbilData>,
    #[serde(rename = "abilitySlot2")]
    pub(in crate::phb) abil2: Option<PItemFighterAbilData>,
}
impl FsdMerge<rc::ed::EItemFighterAbil> for PItemFighterAbils {
    fn fsd_merge(self, id: rc::ReeInt) -> Vec<rc::ed::EItemFighterAbil> {
        let mut vec = Vec::new();
        for p_abil_data_opt in vec![self.abil0, self.abil1, self.abil2].into_iter() {
            if let Some(p_abil_data) = p_abil_data_opt {
                let (charge_count, charge_rearm_time) = p_abil_data
                    .charges
                    .map_or((None, None), |v| (Some(v.count), Some(v.rearm_time)));
                vec.push(rc::ed::EItemFighterAbil::new(
                    id,
                    p_abil_data.abil_id,
                    p_abil_data.cooldown,
                    charge_count,
                    charge_rearm_time,
                ));
            }
        }
        vec
    }
}

#[derive(Debug, serde::Deserialize)]
pub(in crate::phb) struct PItemFighterAbilData {
    #[serde(rename = "abilityID")]
    pub(in crate::phb) abil_id: rc::ReeInt,
    #[serde(rename = "cooldownSeconds")]
    pub(in crate::phb) cooldown: Option<rc::ReeFloat>,
    pub(in crate::phb) charges: Option<PItemFighterAbilChargeData>,
}

#[derive(Debug, serde::Deserialize)]
pub(in crate::phb) struct PItemFighterAbilChargeData {
    #[serde(rename = "chargeCount")]
    pub(in crate::phb) count: rc::ReeInt,
    #[serde(rename = "rearmTimeSeconds")]
    pub(in crate::phb) rearm_time: rc::ReeFloat,
}
