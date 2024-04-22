#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFighterInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
}
impl From<&rc::SolFighterInfo> for HFighterInfoId {
    fn from(core_fighter_info: &rc::SolFighterInfo) -> Self {
        Self {
            id: core_fighter_info.id,
        }
    }
}
