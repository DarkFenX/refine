#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HServiceInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
}
impl From<&rc::SolServiceInfo> for HServiceInfoId {
    fn from(core_service_info: &rc::SolServiceInfo) -> Self {
        Self {
            id: core_service_info.id,
        }
    }
}
