use crate::{
    ad::AItemListId,
    svc::calc::LocationKind,
    ud::{UItem, UItemKey, UShip, UShipKind},
};

impl UItem {
    pub(in crate::svc::calc) fn get_root_loc_kind(&self) -> Option<LocationKind> {
        match self {
            Self::Character(_) => Some(LocationKind::Character),
            Self::Ship(ship) => match ship.get_kind() {
                UShipKind::Ship => Some(LocationKind::Ship),
                UShipKind::Structure => Some(LocationKind::Structure),
                _ => None,
            },
            _ => None,
        }
    }
    pub(in crate::svc::calc) fn get_ship_loc_kind(&self) -> Option<LocationKind> {
        match self {
            Self::Ship(ship) => match ship.get_kind() {
                UShipKind::Ship => Some(LocationKind::Ship),
                UShipKind::Structure => Some(LocationKind::Structure),
                _ => None,
            },
            _ => None,
        }
    }
    pub(in crate::svc::calc) fn is_on_char_root(&self) -> bool {
        matches!(self, Self::Booster(_) | Self::Implant(_) | Self::Skill(_))
    }
    pub(in crate::svc::calc) fn is_on_ship_root(&self) -> bool {
        matches!(
            self,
            Self::Charge(_) | Self::Module(_) | Self::Rig(_) | Self::Service(_) | Self::Stance(_) | Self::Subsystem(_)
        )
    }
    pub(in crate::svc::calc) fn is_on_struct_root(&self) -> bool {
        matches!(
            self,
            Self::Charge(_) | Self::Module(_) | Self::Rig(_) | Self::Service(_) | Self::Stance(_) | Self::Subsystem(_)
        )
    }
    pub(in crate::svc::calc) fn is_owner_modifiable(&self) -> bool {
        matches!(self, Self::Charge(_) | Self::Drone(_) | Self::Fighter(_))
    }
    pub(in crate::svc::calc) fn is_ship_buffable_by_item_list(&self, item_list_id: &AItemListId) -> Option<&UShip> {
        match self {
            Self::Ship(ship) => match ship.get_buff_item_lists() {
                Some(buff_item_lists) => match buff_item_lists.contains(item_list_id) {
                    true => Some(ship),
                    false => None,
                },
                None => None,
            },
            _ => None,
        }
    }
    pub(in crate::svc::calc) fn is_item_buffable_by_item_list(&self, item_list_id: &AItemListId) -> bool {
        match self.get_item_buff_item_lists() {
            Some(buff_item_list_ids) => buff_item_list_ids.contains(item_list_id),
            None => false,
        }
    }
    pub(in crate::svc::calc) fn get_item_buff_item_lists(&self) -> Option<&Vec<AItemListId>> {
        match self {
            Self::Drone(drone) => drone.get_buff_item_lists(),
            Self::Fighter(fighter) => fighter.get_buff_item_lists(),
            Self::Ship(ship) => ship.get_buff_item_lists(),
            _ => None,
        }
    }
    pub(in crate::svc::calc) fn get_other_key(&self) -> Option<UItemKey> {
        match self {
            Self::Charge(charge) => Some(charge.get_cont_item_key()),
            Self::Module(module) => module.get_charge_key(),
            _ => None,
        }
    }
}
