#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::validation) struct HValShipStanceFail {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    stance_item_id: rc::ItemId,
}
impl From<&rc::val::ValShipStanceFail> for HValShipStanceFail {
    fn from(core_val_fail: &rc::val::ValShipStanceFail) -> Self {
        Self {
            stance_item_id: core_val_fail.stance_item_id,
        }
    }
}
