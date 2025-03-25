#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HServiceInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
}
impl From<&rc::ServiceInfo> for HServiceInfoId {
    fn from(core_service_info: &rc::ServiceInfo) -> Self {
        Self {
            id: core_service_info.id,
        }
    }
}
