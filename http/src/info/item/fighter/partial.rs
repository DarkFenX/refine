use crate::shared::HState;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFighterInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::SolFitId,
    pub(crate) type_id: rc::EItemId,
    pub(crate) state: HState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) amt_override: Option<rc::Amount>,
}
impl From<&rc::SolFighterInfo> for HFighterInfoPartial {
    fn from(core_fighter_info: &rc::SolFighterInfo) -> Self {
        Self {
            id: core_fighter_info.id,
            fit_id: core_fighter_info.fit_id,
            type_id: core_fighter_info.a_item_id,
            state: (&core_fighter_info.state).into(),
            amt_override: core_fighter_info.amt_override,
        }
    }
}
