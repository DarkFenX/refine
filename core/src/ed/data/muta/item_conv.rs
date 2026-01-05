use crate::{ed::EItemId, util::LibNamed};

pub struct EMutaItemConv {
    pub muta_id: EItemId,
    pub in_item_id: EItemId,
    pub out_item_id: EItemId,
}
impl LibNamed for EMutaItemConv {
    fn lib_get_name() -> &'static str {
        "EMutaItemConv"
    }
}
