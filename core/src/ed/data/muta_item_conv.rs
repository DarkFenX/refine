use crate::{ed::EItemId, util::Named};

pub struct EMutaItemConv {
    pub muta_id: EItemId,
    pub in_item_id: EItemId,
    pub out_item_id: EItemId,
}
impl Named for EMutaItemConv {
    fn get_name() -> &'static str {
        "EMutaItemConv"
    }
}
