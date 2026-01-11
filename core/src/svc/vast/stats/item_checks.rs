use crate::{
    svc::err::{StatItemCheckError, UItemKindVsStatError, UItemLoadedError},
    ud::{UData, UItem, UItemId, UShip, UShipKind},
};

pub(super) fn check_character(u_data: &UData, item_uid: UItemId) -> Result<(), StatItemCheckError> {
    let item = u_data.items.get(item_uid);
    let is_loaded = match item {
        UItem::Character(u_character) => u_character.is_loaded(),
        _ => return Err(UItemKindVsStatError { item_uid }.into()),
    };
    match is_loaded {
        true => Ok(()),
        false => Err(UItemLoadedError { item_uid }.into()),
    }
}

pub(super) fn check_ship(u_data: &UData, item_uid: UItemId) -> Result<&UShip, StatItemCheckError> {
    let item = u_data.items.get(item_uid);
    let ship = match item {
        UItem::Ship(ship) => ship,
        _ => return Err(UItemKindVsStatError { item_uid }.into()),
    };
    match ship.is_loaded() {
        true => Ok(ship),
        false => Err(UItemLoadedError { item_uid }.into()),
    }
}

pub(super) fn check_ship_no_struct(u_data: &UData, item_uid: UItemId) -> Result<&UShip, StatItemCheckError> {
    let item = u_data.items.get(item_uid);
    let ship = match item {
        UItem::Ship(ship) => match ship.get_kind() {
            UShipKind::Ship | UShipKind::Unknown => ship,
            UShipKind::Structure => return Err(UItemKindVsStatError { item_uid }.into()),
        },
        _ => return Err(UItemKindVsStatError { item_uid }.into()),
    };
    match ship.is_loaded() {
        true => Ok(ship),
        false => Err(UItemLoadedError { item_uid }.into()),
    }
}

pub(super) fn check_fighter_ship(u_data: &UData, item_uid: UItemId) -> Result<(), StatItemCheckError> {
    let item = u_data.items.get(item_uid);
    let is_loaded = match item {
        UItem::Fighter(fighter) => fighter.is_loaded(),
        UItem::Ship(ship) => ship.is_loaded(),
        _ => return Err(UItemKindVsStatError { item_uid }.into()),
    };
    match is_loaded {
        true => Ok(()),
        false => Err(UItemLoadedError { item_uid }.into()),
    }
}

pub(super) fn check_fighter_ship_no_struct(u_data: &UData, item_uid: UItemId) -> Result<(), StatItemCheckError> {
    let item = u_data.items.get(item_uid);
    let is_loaded = match item {
        UItem::Fighter(fighter) => fighter.is_loaded(),
        UItem::Ship(ship) => match ship.get_kind() {
            UShipKind::Ship | UShipKind::Unknown => ship.is_loaded(),
            UShipKind::Structure => return Err(UItemKindVsStatError { item_uid }.into()),
        },
        _ => return Err(UItemKindVsStatError { item_uid }.into()),
    };
    match is_loaded {
        true => Ok(()),
        false => Err(UItemLoadedError { item_uid }.into()),
    }
}

pub(super) fn check_drone_fighter_ship(u_data: &UData, item_uid: UItemId) -> Result<&UItem, StatItemCheckError> {
    let item = u_data.items.get(item_uid);
    let is_loaded = match item {
        UItem::Drone(drone) => drone.is_loaded(),
        UItem::Fighter(fighter) => fighter.is_loaded(),
        UItem::Ship(ship) => ship.is_loaded(),
        _ => return Err(UItemKindVsStatError { item_uid }.into()),
    };
    match is_loaded {
        true => Ok(item),
        false => Err(UItemLoadedError { item_uid }.into()),
    }
}

pub(super) fn check_drone_fighter_ship_no_struct(u_data: &UData, item_uid: UItemId) -> Result<(), StatItemCheckError> {
    let item = u_data.items.get(item_uid);
    let is_loaded = match item {
        UItem::Drone(drone) => drone.is_loaded(),
        UItem::Fighter(fighter) => fighter.is_loaded(),
        UItem::Ship(ship) => match ship.get_kind() {
            UShipKind::Ship | UShipKind::Unknown => ship.is_loaded(),
            UShipKind::Structure => return Err(UItemKindVsStatError { item_uid }.into()),
        },
        _ => return Err(UItemKindVsStatError { item_uid }.into()),
    };
    match is_loaded {
        true => Ok(()),
        false => Err(UItemLoadedError { item_uid }.into()),
    }
}

pub(super) fn check_drone_fighter_module(u_data: &UData, item_uid: UItemId) -> Result<(), StatItemCheckError> {
    let item = u_data.items.get(item_uid);
    let is_loaded = match item {
        UItem::Drone(drone) => drone.is_loaded(),
        UItem::Fighter(fighter) => fighter.is_loaded(),
        UItem::Module(module) => module.is_loaded(),
        _ => return Err(UItemKindVsStatError { item_uid }.into()),
    };
    match is_loaded {
        true => Ok(()),
        false => Err(UItemLoadedError { item_uid }.into()),
    }
}

pub(super) fn check_drone_module(u_data: &UData, item_uid: UItemId) -> Result<(), StatItemCheckError> {
    let item = u_data.items.get(item_uid);
    let is_loaded = match item {
        UItem::Drone(drone) => drone.is_loaded(),
        UItem::Module(module) => module.is_loaded(),
        _ => return Err(UItemKindVsStatError { item_uid }.into()),
    };
    match is_loaded {
        true => Ok(()),
        false => Err(UItemLoadedError { item_uid }.into()),
    }
}

pub(super) fn check_charge_drone_fighter_module(u_data: &UData, item_uid: UItemId) -> Result<(), StatItemCheckError> {
    let item = u_data.items.get(item_uid);
    let is_loaded = match item {
        UItem::Charge(charge) => charge.is_loaded(),
        UItem::Drone(drone) => drone.is_loaded(),
        UItem::Fighter(fighter) => fighter.is_loaded(),
        UItem::Module(module) => module.is_loaded(),
        _ => return Err(UItemKindVsStatError { item_uid }.into()),
    };
    match is_loaded {
        true => Ok(()),
        false => Err(UItemLoadedError { item_uid }.into()),
    }
}

pub(super) fn check_autocharge_charge_drone_fighter_module(
    u_data: &UData,
    item_uid: UItemId,
) -> Result<(), StatItemCheckError> {
    let item = u_data.items.get(item_uid);
    let is_loaded = match item {
        UItem::Autocharge(autocharge) => autocharge.is_loaded(),
        UItem::Charge(charge) => charge.is_loaded(),
        UItem::Drone(drone) => drone.is_loaded(),
        UItem::Fighter(fighter) => fighter.is_loaded(),
        UItem::Module(module) => module.is_loaded(),
        _ => return Err(UItemKindVsStatError { item_uid }.into()),
    };
    match is_loaded {
        true => Ok(()),
        false => Err(UItemLoadedError { item_uid }.into()),
    }
}
