use crate::sol::{
    ItemKey,
    svc::calc::LocationKind,
    uad::item::{Item, ShipKind},
};

impl Item {
    pub(in crate::sol::svc::calc) fn get_root_loc_kind(&self) -> Option<LocationKind> {
        match self {
            Self::Character(_) => Some(LocationKind::Character),
            Self::Ship(ship) => match ship.get_kind() {
                ShipKind::Ship => Some(LocationKind::Ship),
                ShipKind::Structure => Some(LocationKind::Structure),
                _ => None,
            },
            _ => None,
        }
    }
    pub(in crate::sol::svc::calc) fn is_on_char_root(&self) -> bool {
        matches!(self, Self::Booster(_) | Self::Implant(_) | Self::Skill(_))
    }
    pub(in crate::sol::svc::calc) fn is_on_ship_root(&self) -> bool {
        matches!(
            self,
            Self::Charge(_) | Self::Module(_) | Self::Rig(_) | Self::Service(_) | Self::Stance(_) | Self::Subsystem(_)
        )
    }
    pub(in crate::sol::svc::calc) fn is_on_struct_root(&self) -> bool {
        matches!(
            self,
            Self::Charge(_) | Self::Module(_) | Self::Rig(_) | Self::Service(_) | Self::Stance(_) | Self::Subsystem(_)
        )
    }
    pub(in crate::sol::svc::calc) fn is_owner_modifiable(&self) -> bool {
        matches!(self, Self::Charge(_) | Self::Drone(_) | Self::Fighter(_))
    }
    pub(in crate::sol::svc::calc) fn is_buffable(&self) -> bool {
        match self {
            Self::Drone(_) | Self::Fighter(_) => true,
            Self::Ship(ship) => match ship.get_kind() {
                ShipKind::Ship => true,
                ShipKind::Structure => false,
                ShipKind::Unknown => false,
            },
            _ => false,
        }
    }
    pub(in crate::sol::svc::calc) fn get_other(&self) -> Option<ItemKey> {
        match self {
            Self::Charge(charge) => Some(charge.get_cont_item_key()),
            Self::Module(module) => module.get_charge_item_key(),
            _ => None,
        }
    }
}
