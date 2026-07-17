use crate::{
    ItemId,
    stats::{StatOutRepItemKinds, StatTimeOptions},
};

#[derive(Copy, Clone, Default)]
pub struct StatOptionFitOutRps {
    pub item_kinds: StatOutRepItemKinds = StatOutRepItemKinds { .. },
    pub time_options: StatTimeOptions = StatTimeOptions::default(),
    pub projectee_item_id: Option<ItemId> = None,
}

#[derive(Copy, Clone, Default)]
pub struct StatOptionItemOutRps {
    pub time_options: StatTimeOptions = StatTimeOptions::default(),
    pub ignore_state: bool,
    pub projectee_item_id: Option<ItemId> = None,
}
