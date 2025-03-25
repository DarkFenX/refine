use std::collections::HashMap;

use crate::shared::HEffectId;

use super::side_effect::HSideEffectInfo;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HBoosterInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::ItemTypeId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::FitId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) slot: Option<rc::SlotIndex>,
    pub(crate) enabled: bool,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub(crate) side_effects: HashMap<HEffectId, HSideEffectInfo>,
}
impl HBoosterInfoPartial {
    pub(super) fn mk_info(core_sol: &mut rc::SolarSystem, core_booster_info: &rc::BoosterInfo) -> Self {
        let mut side_effects = HashMap::new();
        for (effect_id, core_se_info) in core_booster_info.side_effects.iter() {
            let se_info = HSideEffectInfo::from_core_info(core_sol, &core_booster_info.id, core_se_info);
            side_effects.insert(effect_id.into(), se_info);
        }
        Self {
            id: core_booster_info.id,
            kind: "booster",
            type_id: core_booster_info.type_id,
            fit_id: core_booster_info.fit_id,
            slot: core_booster_info.slot,
            enabled: core_booster_info.enabled,
            side_effects,
        }
    }
}
