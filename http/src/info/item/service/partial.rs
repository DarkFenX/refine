use rc::ItemCommon;

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
impl From<&mut rc::ServiceMut<'_>> for HServiceInfoPartial {
    fn from(core_service: &mut rc::ServiceMut) -> Self {
        Self {
            id: core_service.get_item_id(),
            kind: "service",
            type_id: core_service.get_type_id(),
            fit_id: core_service.get_fit().get_fit_id(),
            enabled: (&core_service.get_state()).into(),
        }
    }
}
