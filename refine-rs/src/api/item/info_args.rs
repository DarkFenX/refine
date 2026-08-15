use crate::ItemInfoModes;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct ItemInfoArgs {
    #[cfg_attr(feature = "serde", serde(default))]
    pub item: ItemInfoModes = ItemInfoModes::default(),
}
