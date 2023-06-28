use crate::{
    defs::{EItemId, EMutaId},
    util::Named,
};

/// EVE mutaplasmid item type conversion data.
#[derive(Debug)]
pub struct EMutaItemConv {
    /// Mutaplasmid item type ID.
    pub muta_id: EMutaId,
    /// Refers an item type the mutaplasmid can be applied to.
    pub in_item_id: EItemId,
    /// Refers an item type, which is the outcome of the conversion.
    pub out_item_id: EItemId,
}
impl EMutaItemConv {
    /// Make a new EVE mutaplasmid item type conversion.
    pub fn new(muta_id: EMutaId, in_item_id: EItemId, out_item_id: EItemId) -> Self {
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
