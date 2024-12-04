use crate::{defs::EItemId, util::Named};

/// EVE mutator item type conversion data.
pub struct EMutaItemConv {
    /// Mutator item type ID.
    pub muta_id: EItemId,
    /// Refers an item type the mutator can be applied to.
    pub in_item_id: EItemId,
    /// Refers an item type, which is the outcome of the conversion.
    pub out_item_id: EItemId,
}
impl EMutaItemConv {
    /// Make a new EVE mutator item type conversion.
    pub fn new(muta_id: EItemId, in_item_id: EItemId, out_item_id: EItemId) -> Self {
        Self {
            muta_id,
            in_item_id,
            out_item_id,
        }
    }
}
impl Named for EMutaItemConv {
    fn get_name() -> &'static str {
        "EMutaItemConv"
    }
}
