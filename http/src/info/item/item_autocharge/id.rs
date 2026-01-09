use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HAutochargeInfoId {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
}
impl From<&mut rc::AutochargeMut<'_>> for HAutochargeInfoId {
    fn from(core_autocharge: &mut rc::AutochargeMut) -> Self {
        Self {
            id: core_autocharge.get_item_id(),
        }
    }
}
