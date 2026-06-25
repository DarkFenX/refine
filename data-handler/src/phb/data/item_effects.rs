use serde::Deserialize;

use crate::phb::{
    parsing::{Key, KeyMerge},
    serde_custom::bool_from_int,
};

#[derive(Deserialize)]
pub(in crate::phb) struct PItemEffects {
    #[serde(rename = "dogmaEffects", default)]
    pub(in crate::phb) effects: Vec<PItemEffectData>,
}
impl KeyMerge<rc::ed::EItemEffect> for PItemEffects {
    fn key_merge(self, key: Key) -> Vec<rc::ed::EItemEffect> {
        self.effects
            .into_iter()
            .map(|v| rc::ed::EItemEffect {
                item_id: rc::ed::EItemId::from_i32(key),
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
