use crate::{defs::ReeInt, util::Named};

/// EVE mutaplasmid item type conversion data.
#[derive(Debug)]
pub struct EMutaItemConv {
    /// Mutaplasmid item type ID.
    pub muta_id: ReeInt,
    /// Refers an item type the mutaplasmid can be applied to.
    pub in_item_id: ReeInt,
    /// Refers an item type, which is the outcome of the conversion.
    pub out_item_id: ReeInt,
}
impl EMutaItemConv {
    /// Make a new EVE mutaplasmid item type conversion.
    pub fn new(muta_id: ReeInt, in_item_id: ReeInt, out_item_id: ReeInt) -> Self {
        Self {
            muta_id,
            in_item_id,
            out_item_id,
        }
    }
}
impl Named for EMutaItemConv {
    fn get_name() -> &'static str {
        "ed::EMutaItemConv"
    }
}
