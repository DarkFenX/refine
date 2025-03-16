#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HValShipStanceFail {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    item_id: rc::SolItemId,
}
impl From<&rc::SolValShipStanceFail> for HValShipStanceFail {
    fn from(core_val_fail: &rc::SolValShipStanceFail) -> Self {
        Self {
            item_id: core_val_fail.item_id,
        }
    }
}
