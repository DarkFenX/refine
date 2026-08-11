use crate::{
    ed::{EItemId, EMutaAttr},
    util::LibNamed,
};

pub struct EMuta {
    pub id: EItemId,
    pub in_item_ids: Vec<EItemId>,
    pub out_item_id: EItemId,
    pub attrs: Vec<EMutaAttr>,
}
impl LibNamed for EMuta {
    fn lib_get_name() -> &'static str {
        "EMuta"
    }
}
