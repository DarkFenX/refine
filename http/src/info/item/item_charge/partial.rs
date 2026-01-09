use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HChargeInfoPartial {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
    kind: &'static str,
    type_id: i32,
    #[serde_as(as = "DisplayFromStr")]
    fit_id: rc::FitId,
    #[serde_as(as = "DisplayFromStr")]
    cont_item_id: rc::ItemId,
    enabled: bool,
}
impl From<&mut rc::ChargeMut<'_>> for HChargeInfoPartial {
    fn from(core_charge: &mut rc::ChargeMut) -> Self {
        Self {
            id: core_charge.get_item_id(),
            kind: "charge",
            type_id: core_charge.get_type_id().into_i32(),
            fit_id: core_charge.get_fit().get_fit_id(),
            cont_item_id: core_charge.get_cont_item().get_item_id(),
            enabled: core_charge.get_state(),
        }
    }
}
