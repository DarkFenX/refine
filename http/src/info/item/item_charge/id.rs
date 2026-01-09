use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HChargeInfoId {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
}
impl From<&mut rc::ChargeMut<'_>> for HChargeInfoId {
    fn from(core_charge: &mut rc::ChargeMut) -> Self {
        Self {
            id: core_charge.get_item_id(),
        }
    }
}
