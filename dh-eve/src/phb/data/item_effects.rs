use crate::phb::{
    fsd::{FsdId, FsdMerge},
    serde_custom::bool_from_int,
};

#[derive(serde::Deserialize)]
pub(in crate::phb) struct PItemEffects {
    #[serde(rename = "dogmaEffects", default)]
    pub(in crate::phb) effects: Vec<PItemEffectData>,
}
impl FsdMerge<rc::ed::EItemEffect> for PItemEffects {
    fn fsd_merge(self, id: FsdId) -> Vec<rc::ed::EItemEffect> {
        self.effects
            .into_iter()
            .map(|v| rc::ed::EItemEffect::new(id, v.effect_id, v.is_default))
            .collect()
    }
}

#[derive(serde::Deserialize)]
pub(in crate::phb) struct PItemEffectData {
    #[serde(rename = "effectID")]
    pub(in crate::phb) effect_id: rc::EEffectId,
    #[serde(rename = "isDefault", deserialize_with = "bool_from_int")]
    pub(in crate::phb) is_default: bool,
}
