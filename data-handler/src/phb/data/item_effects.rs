use serde::Deserialize;

use crate::phb::{
    fsd::{FsdId, FsdMerge},
    serde_custom::bool_from_int,
};

#[derive(Deserialize)]
pub(in crate::phb) struct PItemEffects {
    #[serde(rename = "dogmaEffects", default)]
    pub(in crate::phb) effects: Vec<PItemEffectData>,
}
impl FsdMerge<rc::ed::EItemEffect> for PItemEffects {
    fn fsd_merge(self, id: FsdId) -> Vec<rc::ed::EItemEffect> {
        self.effects
            .into_iter()
            .map(|v| rc::ed::EItemEffect {
                item_id: rc::ed::EItemId::from_i32(id),
                effect_id: rc::ed::EEffectId::from_i32(v.effect_id),
                is_default: v.is_default,
            })
            .collect()
    }
}

#[derive(Deserialize)]
pub(in crate::phb) struct PItemEffectData {
    #[serde(rename = "effectID")]
    pub(in crate::phb) effect_id: i32,
    #[serde(rename = "isDefault", deserialize_with = "bool_from_int")]
    pub(in crate::phb) is_default: bool,
}
