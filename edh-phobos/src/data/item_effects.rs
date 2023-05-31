use crate::fsd::FsdMerge;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ItemEffects {
    #[serde(rename = "dogmaEffects", default)]
    pub(crate) effects: Vec<ItemEffectData>,
}
impl FsdMerge<rc::edt::ItemEffect> for ItemEffects {
    fn fsd_merge(self, id: rc::ReeInt) -> Vec<rc::edt::ItemEffect> {
        self.effects
            .into_iter()
            .map(|v| rc::edt::ItemEffect::new(id, v.effect_id, v.is_default != 0))
            .collect()
    }
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ItemEffectData {
    #[serde(rename = "effectID")]
    pub(crate) effect_id: rc::ReeInt,
    #[serde(rename = "isDefault")]
    pub(crate) is_default: rc::ReeInt,
}
