use crate::{
    ad::{ACustomItemListId, AItemListId},
    ec::itemlists as ecil,
};

pub(crate) const WORMHOLE_JUMP_BLACK_LIST: AItemListId = ecil::WORMHOLE_JUMP_BLACK_LIST.into();

// Library-specific item lists
pub(crate) const SHIPS: AItemListId = AItemListId::Custom(ACustomItemListId::new(1));
pub(crate) const SHIPS_DRONES_FIGHTERS_ENTITIES: AItemListId = AItemListId::Custom(ACustomItemListId::new(2));
pub(crate) const CAPITALS_FREIGHTERS: AItemListId = AItemListId::Custom(ACustomItemListId::new(3));
pub(crate) const PANIC_ELIGIBLE: AItemListId = AItemListId::Custom(ACustomItemListId::new(4));
