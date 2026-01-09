use serde_tuple::Serialize_tuple;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize_tuple)]
pub(in crate::info::validation) struct HValShipStanceFail {
    #[serde_as(as = "DisplayFromStr")]
    stance_item_id: rc::ItemId,
}
impl From<&rc::val::ValShipStanceFail> for HValShipStanceFail {
    fn from(core_val_fail: &rc::val::ValShipStanceFail) -> Self {
        Self {
            stance_item_id: core_val_fail.stance_item_id,
        }
    }
}
