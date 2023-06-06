use crate::phb::fsd::FsdMerge;

#[derive(Debug, serde::Deserialize)]
pub(in crate::phb) struct PItemEffects {
    #[serde(rename = "dogmaEffects", default)]
    pub(in crate::phb) effects: Vec<PItemEffectData>,
}
impl FsdMerge<rc::ed::EItemEffect> for PItemEffects {
    fn fsd_merge(self, id: rc::ReeInt) -> Vec<rc::ed::EItemEffect> {
        self.effects
            .into_iter()
            .map(|v| rc::ed::EItemEffect::new(id, v.effect_id, v.is_default != 0))
            .collect()
    }
}

#[derive(Debug, serde::Deserialize)]
pub(in crate::phb) struct PItemEffectData {
    #[serde(rename = "effectID")]
    pub(in crate::phb) effect_id: rc::ReeInt,
    #[serde(rename = "isDefault")]
    pub(in crate::phb) is_default: rc::ReeInt,
}
