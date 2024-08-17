use std::collections::HashMap;

use super::side_effect::HSideEffectInfo;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HBoosterInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::EItemId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::SolFitId,
    pub(crate) enabled: bool,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub(crate) side_effects: HashMap<rc::EEffectId, HSideEffectInfo>,
}
impl HBoosterInfoPartial {
    pub(super) fn mk_info(core_sol: &mut rc::SolarSystem, core_booster_info: &rc::SolBoosterInfo) -> Self {
        let mut side_effects = HashMap::new();
        for (effect_id, core_se_info) in core_booster_info.side_effects.iter() {
            if let Some(se_info) = HSideEffectInfo::from_core_info(core_sol, &core_booster_info.id, core_se_info) {
                side_effects.insert(*effect_id, se_info);
            }
        }
        Self {
            id: core_booster_info.id,
            kind: "booster",
            type_id: core_booster_info.type_id,
            fit_id: core_booster_info.fit_id,
            enabled: core_booster_info.enabled,
            side_effects,
        }
    }
}
