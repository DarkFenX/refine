use rc::ItemCommon;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HChargeInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::ItemId,
}
impl From<&mut rc::ChargeMut<'_>> for HChargeInfoId {
    fn from(core_charge: &mut rc::ChargeMut) -> Self {
        Self {
            id: core_charge.get_item_id(),
        }
    }
}
