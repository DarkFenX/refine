use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFighterInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
    #[serde_as(as = "HashMap<_, serde_with::DisplayFromStr>")]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub(crate) autocharges: HashMap<rc::EEffectId, rc::SolItemId>,
}
impl From<&rc::SolFighterInfo> for HFighterInfoId {
    fn from(core_fighter_info: &rc::SolFighterInfo) -> Self {
        Self {
            id: core_fighter_info.id,
            autocharges: core_fighter_info
                .autocharges
                .iter()
                .map(|(k, v)| (*k, v.id))
                .collect(),
        }
    }
}
