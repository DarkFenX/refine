use crate::{util::Named, ReeInt};

/// Mutaplasmid item type conversion data.
#[derive(Debug)]
pub struct MutaItemConv {
    /// Mutaplasmid item type ID.
    pub muta_id: ReeInt,
    /// Refers an item type the mutaplasmid can be applied to.
    pub in_item_id: ReeInt,
    /// Refers an item type, which is the outcome of the conversion.
    pub out_item_id: ReeInt,
}
impl MutaItemConv {
    /// Make a new mutaplasmid item type conversion.
    pub fn new(muta_id: ReeInt, in_item_id: ReeInt, out_item_id: ReeInt) -> MutaItemConv {
        MutaItemConv {
            muta_id,
            in_item_id,
            out_item_id,
        }
    }
}
impl Named for MutaItemConv {
    fn get_name() -> &'static str {
        "dh::MutaItemConv"
    }
}
