// TODO: after everything is implemented, remove this and see what needs to be cleaned up
#![allow(dead_code)]

use crate::{ad::AItemListId, ed::consts::itemlists as ecil};

pub(crate) const WORMHOLE_JUMP_BLACK_LIST: AItemListId = AItemListId::Eve(ecil::WORMHOLE_JUMP_BLACK_LIST);

// Library-specific item lists
pub(crate) const SHIPS: AItemListId = AItemListId::Custom(1);
pub(crate) const SHIPS_DRONES_FIGHTERS_NPCS: AItemListId = AItemListId::Custom(2);
