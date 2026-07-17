use crate::{
    ItemId,
    stats::{StatNeutItemKinds, StatTimeOptions},
};

#[derive(Copy, Clone, Default)]
pub struct StatOptionFitOutNps {
    pub item_kinds: StatNeutItemKinds = StatNeutItemKinds { .. },
    pub time_options: StatTimeOptions = StatTimeOptions::default(),
    pub projectee_item_id: Option<ItemId> = None,
}

#[derive(Copy, Clone, Default)]
pub struct StatOptionItemOutNps {
    pub time_options: StatTimeOptions = StatTimeOptions::default(),
    pub include_charges: bool = false,
    pub ignore_state: bool = false,
    pub projectee_item_id: Option<ItemId> = None,
}
