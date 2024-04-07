#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HFighterInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SsItemId,
}
impl From<&rc::SsFighterInfo> for HFighterInfoId {
    fn from(core_fighter_info: &rc::SsFighterInfo) -> Self {
        Self {
            id: core_fighter_info.id,
        }
    }
}
