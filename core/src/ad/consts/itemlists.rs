use crate::{ad::AItemListId, ec::itemlists as ecil};

pub(crate) const WORMHOLE_JUMP_BLACK_LIST: AItemListId = AItemListId::Eve(ecil::WORMHOLE_JUMP_BLACK_LIST);

// Library-specific item lists
pub(crate) const SHIPS: AItemListId = AItemListId::Custom(1);
pub(crate) const SHIPS_DRONES_FIGHTERS_NPCS: AItemListId = AItemListId::Custom(2);
pub(crate) const CAPITALS_FREIGHTERS: AItemListId = AItemListId::Custom(3);
