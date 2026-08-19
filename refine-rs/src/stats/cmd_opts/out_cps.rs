use crate::{
    ItemId,
    stats::{StatItemStateOptions, StatTimeOptions},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone)]
pub struct StatOptionFitOutCps<I = ItemId> {
    #[cfg_attr(feature = "serde", serde(default))]
    pub time: StatTimeOptions = StatTimeOptions::default(),
    pub projectee_item_id: Option<I> = None,
}
impl<I> Default for StatOptionFitOutCps<I> {
    fn default() -> Self {
        Self { .. }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone)]
pub struct StatOptionItemOutCps<I = ItemId> {
    #[cfg_attr(feature = "serde", serde(default))]
    pub time: StatTimeOptions = StatTimeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub state: StatItemStateOptions = StatItemStateOptions::default(),
    pub projectee_item_id: Option<I> = None,
}
impl<I> Default for StatOptionItemOutCps<I> {
    fn default() -> Self {
        Self { .. }
    }
}
