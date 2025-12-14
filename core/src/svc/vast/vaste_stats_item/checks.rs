use crate::{
    svc::err::{KeyedItemKindVsStatError, KeyedItemLoadedError, StatItemCheckError},
    ud::{UData, UItem, UItemKey, UShip, UShipKind},
};

pub(super) fn check_character(u_data: &UData, item_key: UItemKey) -> Result<(), StatItemCheckError> {
    let item = u_data.items.get(item_key);
    let is_loaded = match item {
        UItem::Character(u_character) => u_character.is_loaded(),
        _ => return Err(KeyedItemKindVsStatError { item_key }.into()),
    };
    match is_loaded {
        true => Ok(()),
        false => Err(KeyedItemLoadedError { item_key }.into()),
    }
}

pub(super) fn check_ship(u_data: &UData, item_key: UItemKey) -> Result<&UShip, StatItemCheckError> {
    let item = u_data.items.get(item_key);
    let ship = match item {
        UItem::Ship(ship) => ship,
        _ => return Err(KeyedItemKindVsStatError { item_key }.into()),
    };
    match ship.is_loaded() {
        true => Ok(ship),
        false => Err(KeyedItemLoadedError { item_key }.into()),
    }
}

pub(super) fn check_ship_no_struct(u_data: &UData, item_key: UItemKey) -> Result<&UShip, StatItemCheckError> {
    let item = u_data.items.get(item_key);
    let ship = match item {
        UItem::Ship(ship) => match ship.get_kind() {
            UShipKind::Ship | UShipKind::Unknown => ship,
            UShipKind::Structure => return Err(KeyedItemKindVsStatError { item_key }.into()),
        },
        _ => return Err(KeyedItemKindVsStatError { item_key }.into()),
    };
    match ship.is_loaded() {
        true => Ok(ship),
        false => Err(KeyedItemLoadedError { item_key }.into()),
    }
}

pub(super) fn check_fighter_ship(u_data: &UData, item_key: UItemKey) -> Result<(), StatItemCheckError> {
    let item = u_data.items.get(item_key);
    let is_loaded = match item {
        UItem::Fighter(fighter) => fighter.is_loaded(),
        UItem::Ship(ship) => ship.is_loaded(),
        _ => return Err(KeyedItemKindVsStatError { item_key }.into()),
    };
    match is_loaded {
        true => Ok(()),
        false => Err(KeyedItemLoadedError { item_key }.into()),
    }
}

pub(super) fn check_fighter_ship_no_struct(u_data: &UData, item_key: UItemKey) -> Result<(), StatItemCheckError> {
    let item = u_data.items.get(item_key);
    let is_loaded = match item {
        UItem::Fighter(fighter) => fighter.is_loaded(),
        UItem::Ship(ship) => match ship.get_kind() {
            UShipKind::Ship | UShipKind::Unknown => ship.is_loaded(),
            UShipKind::Structure => return Err(KeyedItemKindVsStatError { item_key }.into()),
        },
        _ => return Err(KeyedItemKindVsStatError { item_key }.into()),
    };
    match is_loaded {
        true => Ok(()),
        false => Err(KeyedItemLoadedError { item_key }.into()),
    }
}

pub(super) fn check_drone_fighter_ship(u_data: &UData, item_key: UItemKey) -> Result<&UItem, StatItemCheckError> {
    let item = u_data.items.get(item_key);
    let is_loaded = match item {
        UItem::Drone(drone) => drone.is_loaded(),
        UItem::Fighter(fighter) => fighter.is_loaded(),
        UItem::Ship(ship) => ship.is_loaded(),
        _ => return Err(KeyedItemKindVsStatError { item_key }.into()),
    };
    match is_loaded {
        true => Ok(item),
        false => Err(KeyedItemLoadedError { item_key }.into()),
    }
}

pub(super) fn check_drone_fighter_ship_no_struct(u_data: &UData, item_key: UItemKey) -> Result<(), StatItemCheckError> {
    let item = u_data.items.get(item_key);
    let is_loaded = match item {
        UItem::Drone(drone) => drone.is_loaded(),
        UItem::Fighter(fighter) => fighter.is_loaded(),
        UItem::Ship(ship) => match ship.get_kind() {
            UShipKind::Ship | UShipKind::Unknown => ship.is_loaded(),
            UShipKind::Structure => return Err(KeyedItemKindVsStatError { item_key }.into()),
        },
        _ => return Err(KeyedItemKindVsStatError { item_key }.into()),
    };
    match is_loaded {
        true => Ok(()),
        false => Err(KeyedItemLoadedError { item_key }.into()),
    }
}

pub(super) fn check_drone_fighter_module(u_data: &UData, item_key: UItemKey) -> Result<(), StatItemCheckError> {
    let item = u_data.items.get(item_key);
    let is_loaded = match item {
        UItem::Drone(drone) => drone.is_loaded(),
        UItem::Fighter(fighter) => fighter.is_loaded(),
        UItem::Module(module) => module.is_loaded(),
        _ => return Err(KeyedItemKindVsStatError { item_key }.into()),
    };
    match is_loaded {
        true => Ok(()),
        false => Err(KeyedItemLoadedError { item_key }.into()),
    }
}

pub(super) fn check_drone_module(u_data: &UData, item_key: UItemKey) -> Result<(), StatItemCheckError> {
    let item = u_data.items.get(item_key);
    let is_loaded = match item {
        UItem::Drone(drone) => drone.is_loaded(),
        UItem::Module(module) => module.is_loaded(),
        _ => return Err(KeyedItemKindVsStatError { item_key }.into()),
    };
    match is_loaded {
        true => Ok(()),
        false => Err(KeyedItemLoadedError { item_key }.into()),
    }
}

pub(super) fn check_charge_drone_fighter_module(u_data: &UData, item_key: UItemKey) -> Result<(), StatItemCheckError> {
    let item = u_data.items.get(item_key);
    let is_loaded = match item {
        UItem::Charge(charge) => charge.is_loaded(),
        UItem::Drone(drone) => drone.is_loaded(),
        UItem::Fighter(fighter) => fighter.is_loaded(),
        UItem::Module(module) => module.is_loaded(),
        _ => return Err(KeyedItemKindVsStatError { item_key }.into()),
    };
    match is_loaded {
        true => Ok(()),
        false => Err(KeyedItemLoadedError { item_key }.into()),
    }
}

pub(super) fn check_autocharge_charge_drone_fighter_module(
    u_data: &UData,
    item_key: UItemKey,
) -> Result<(), StatItemCheckError> {
    let item = u_data.items.get(item_key);
    let is_loaded = match item {
        UItem::Autocharge(autocharge) => autocharge.is_loaded(),
        UItem::Charge(charge) => charge.is_loaded(),
        UItem::Drone(drone) => drone.is_loaded(),
        UItem::Fighter(fighter) => fighter.is_loaded(),
        UItem::Module(module) => module.is_loaded(),
        _ => return Err(KeyedItemKindVsStatError { item_key }.into()),
    };
    match is_loaded {
        true => Ok(()),
        false => Err(KeyedItemLoadedError { item_key }.into()),
    }
}
