use super::HServiceInfoPartial;
use crate::info::item::extended::HItemExtendedInfo;

#[derive(serde::Serialize)]
pub(crate) struct HServiceInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HServiceInfoPartial,
    #[serde(flatten)]
    pub(crate) extended_info: HItemExtendedInfo,
}
impl From<&mut rc::ServiceMut<'_>> for HServiceInfoFull {
    fn from(core_service: &mut rc::ServiceMut) -> Self {
        Self {
            partial_info: core_service.into(),
            extended_info: core_service.into(),
        }
    }
}
