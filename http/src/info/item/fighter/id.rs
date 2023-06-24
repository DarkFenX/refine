#[derive(serde::Serialize)]
pub(crate) struct HFighterInfoId {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::ReeId,
}
impl From<&rc::SsFighterInfo> for HFighterInfoId {
    fn from(core_fighter_info: &rc::SsFighterInfo) -> Self {
        Self {
            id: core_fighter_info.id,
        }
    }
}
