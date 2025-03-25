use crate::shared::HServiceState;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HServiceInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::ItemTypeId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::FitId,
    pub(crate) enabled: HServiceState,
}
impl From<&rc::ServiceInfo> for HServiceInfoPartial {
    fn from(core_service_info: &rc::ServiceInfo) -> Self {
        Self {
            id: core_service_info.id,
            kind: "service",
            type_id: core_service_info.type_id,
            fit_id: core_service_info.fit_id,
            enabled: (&core_service_info.state).into(),
        }
    }
}
