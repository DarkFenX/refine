use crate::{InfoModes, ItemId, ItemIdBackref, info::InfoModesInt};

#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone)]
pub enum ItemInfoMode {
    Id,
    Partial,
    Full,
}
const impl Default for ItemInfoMode {
    fn default() -> Self {
        Self::Partial
    }
}

pub type ItemInfoModes = InfoModes<ItemInfoMode, ItemId>;
pub type ItemInfoModesBackref = InfoModes<ItemInfoMode, ItemIdBackref>;
pub(crate) type ItemInfoModesInt = InfoModesInt<ItemInfoMode, ItemId>;
