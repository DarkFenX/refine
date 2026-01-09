use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HServiceInfoId {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
}
impl From<&mut rc::ServiceMut<'_>> for HServiceInfoId {
    fn from(core_service: &mut rc::ServiceMut) -> Self {
        Self {
            id: core_service.get_item_id(),
        }
    }
}
