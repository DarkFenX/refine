use crate::{ed::EItemId, util::Named};

/// EVE mutator item type conversion data.
pub struct EMutaItemConv {
    /// Mutator item type ID.
    pub muta_id: EItemId,
    /// Refers an item type the mutator can be applied to.
    pub in_item_id: EItemId,
    /// Refers an item type, which is the outcome of the conversion.
    pub out_item_id: EItemId,
}
impl Named for EMutaItemConv {
    fn get_name() -> &'static str {
        "EMutaItemConv"
    }
}
