use crate::{
    ItemId,
    stats::{StatItemChargeOptions, StatItemStateOptions, StatNeutItemKinds, StatTimeOptions},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone)]
pub struct StatOptionFitOutNps<I = ItemId> {
    #[cfg_attr(feature = "serde", serde(default))]
    pub item_kinds: StatNeutItemKinds = StatNeutItemKinds::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub time: StatTimeOptions = StatTimeOptions::default(),
    pub projectee_item_id: Option<I> = None,
}
impl<I> Default for StatOptionFitOutNps<I> {
    fn default() -> Self {
        Self { .. }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone)]
pub struct StatOptionItemOutNps<I = ItemId> {
    #[cfg_attr(feature = "serde", serde(default))]
    pub time: StatTimeOptions = StatTimeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub charges: StatItemChargeOptions = StatItemChargeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub state: StatItemStateOptions = StatItemStateOptions::default(),
    pub projectee_item_id: Option<I> = None,
}
impl<I> Default for StatOptionItemOutNps<I> {
    fn default() -> Self {
        Self { .. }
    }
}
