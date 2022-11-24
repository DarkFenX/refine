use crate::{defines::ReeInt, dh};

use super::super::fsd::FsdMerge;

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct ItemEffects {
    #[serde(rename = "dogmaEffects", default)]
    pub(in super::super) effects: Vec<ItemEffectData>,
}
impl FsdMerge<dh::ItemEffect> for ItemEffects {
    fn fsd_merge(self, id: ReeInt) -> Vec<dh::ItemEffect> {
        self.effects
            .into_iter()
            .map(|v| dh::ItemEffect::new(id, v.effect_id, v.is_default != 0))
            .collect()
    }
}

#[derive(Debug, serde::Deserialize)]
pub(in super::super) struct ItemEffectData {
    #[serde(rename = "effectID")]
    pub(in super::super) effect_id: ReeInt,
    #[serde(rename = "isDefault")]
    pub(in super::super) is_default: ReeInt,
}
