use crate::{
    ad::{ACustomItemListId, AItemListId},
    ed::EItemListId,
};

impl AItemListId {
    pub(crate) const WORMHOLE_JUMP_BLACK_LIST: Self = Self::from_eid(EItemListId::WORMHOLE_JUMP_BLACK_LIST);
    // Library-specific item lists
    pub(crate) const SHIPS: Self = Self::Custom(ACustomItemListId::from_i32(1));
    pub(crate) const SHIPS_DRONES_FIGHTERS: Self = Self::Custom(ACustomItemListId::from_i32(2));
    pub(crate) const SHIPS_DRONES_FIGHTERS_ENTITIES: Self = Self::Custom(ACustomItemListId::from_i32(3));
    pub(crate) const DRONES_FIGHTERS: Self = Self::Custom(ACustomItemListId::from_i32(4));
    pub(crate) const CAPITALS_FREIGHTERS: Self = Self::Custom(ACustomItemListId::from_i32(5));
    pub(crate) const PANIC_ELIGIBLE: Self = Self::Custom(ACustomItemListId::from_i32(6));
}
