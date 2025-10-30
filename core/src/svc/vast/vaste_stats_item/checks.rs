use crate::{
    svc::{
        SvcCtx,
        err::{KeyedItemKindVsStatError, KeyedItemLoadedError, StatItemCheckError},
    },
    ud::{UItem, UItemKey, UShipKind},
};

pub(super) fn check_item_key_character(ctx: SvcCtx, item_key: UItemKey) -> Result<(), StatItemCheckError> {
    let u_item = ctx.u_data.items.get(item_key);
    let is_loaded = match u_item {
        UItem::Character(u_character) => u_character.is_loaded(),
        _ => return Err(KeyedItemKindVsStatError { item_key }.into()),
    };
    match is_loaded {
        true => Ok(()),
        false => Err(KeyedItemLoadedError { item_key }.into()),
    }
}

pub(super) fn check_item_key_ship(ctx: SvcCtx, item_key: UItemKey) -> Result<(), StatItemCheckError> {
    let u_item = ctx.u_data.items.get(item_key);
    let is_loaded = match u_item {
        UItem::Ship(u_ship) => u_ship.is_loaded(),
        _ => return Err(KeyedItemKindVsStatError { item_key }.into()),
    };
    match is_loaded {
        true => Ok(()),
        false => Err(KeyedItemLoadedError { item_key }.into()),
    }
}

pub(super) fn check_item_key_ship_no_struct(ctx: SvcCtx, item_key: UItemKey) -> Result<(), StatItemCheckError> {
    let u_item = ctx.u_data.items.get(item_key);
    let is_loaded = match u_item {
        UItem::Ship(u_ship) => match u_ship.get_kind() {
            UShipKind::Ship | UShipKind::Unknown => u_ship.is_loaded(),
            UShipKind::Structure => return Err(KeyedItemKindVsStatError { item_key }.into()),
        },
        _ => return Err(KeyedItemKindVsStatError { item_key }.into()),
    };
    match is_loaded {
        true => Ok(()),
        false => Err(KeyedItemLoadedError { item_key }.into()),
    }
}

pub(super) fn check_item_key_fighter_ship(ctx: SvcCtx, item_key: UItemKey) -> Result<(), StatItemCheckError> {
    let u_item = ctx.u_data.items.get(item_key);
    let is_loaded = match u_item {
        UItem::Fighter(u_fighter) => u_fighter.is_loaded(),
        UItem::Ship(u_ship) => u_ship.is_loaded(),
        _ => return Err(KeyedItemKindVsStatError { item_key }.into()),
    };
    match is_loaded {
        true => Ok(()),
        false => Err(KeyedItemLoadedError { item_key }.into()),
    }
}

pub(super) fn check_item_key_fighter_ship_no_struct(ctx: SvcCtx, item_key: UItemKey) -> Result<(), StatItemCheckError> {
    let u_item = ctx.u_data.items.get(item_key);
    let is_loaded = match u_item {
        UItem::Fighter(u_fighter) => u_fighter.is_loaded(),
        UItem::Ship(u_ship) => match u_ship.get_kind() {
            UShipKind::Ship | UShipKind::Unknown => u_ship.is_loaded(),
            UShipKind::Structure => return Err(KeyedItemKindVsStatError { item_key }.into()),
        },
        _ => return Err(KeyedItemKindVsStatError { item_key }.into()),
    };
    match is_loaded {
        true => Ok(()),
        false => Err(KeyedItemLoadedError { item_key }.into()),
    }
}

pub(super) fn check_item_key_drone_fighter_ship(ctx: SvcCtx, item_key: UItemKey) -> Result<(), StatItemCheckError> {
    let u_item = ctx.u_data.items.get(item_key);
    check_item_drone_fighter_ship(item_key, u_item)
}

pub(super) fn check_item_drone_fighter_ship(item_key: UItemKey, u_item: &UItem) -> Result<(), StatItemCheckError> {
    let is_loaded = match u_item {
        UItem::Drone(u_drone) => u_drone.is_loaded(),
        UItem::Fighter(u_fighter) => u_fighter.is_loaded(),
        UItem::Ship(u_ship) => u_ship.is_loaded(),
        _ => return Err(KeyedItemKindVsStatError { item_key }.into()),
    };
    match is_loaded {
        true => Ok(()),
        false => Err(KeyedItemLoadedError { item_key }.into()),
    }
}

pub(super) fn check_item_key_drone_fighter_ship_no_struct(
    ctx: SvcCtx,
    item_key: UItemKey,
) -> Result<(), StatItemCheckError> {
    let u_item = ctx.u_data.items.get(item_key);
    let is_loaded = match u_item {
        UItem::Drone(u_drone) => u_drone.is_loaded(),
        UItem::Fighter(u_fighter) => u_fighter.is_loaded(),
        UItem::Ship(u_ship) => match u_ship.get_kind() {
            UShipKind::Ship | UShipKind::Unknown => u_ship.is_loaded(),
            UShipKind::Structure => return Err(KeyedItemKindVsStatError { item_key }.into()),
        },
        _ => return Err(KeyedItemKindVsStatError { item_key }.into()),
    };
    match is_loaded {
        true => Ok(()),
        false => Err(KeyedItemLoadedError { item_key }.into()),
    }
}

pub(super) fn check_item_key_drone_fighter_module(ctx: SvcCtx, item_key: UItemKey) -> Result<(), StatItemCheckError> {
    let u_item = ctx.u_data.items.get(item_key);
    let is_loaded = match u_item {
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

pub(super) fn check_item_key_charge_drone_fighter_module(
    ctx: SvcCtx,
    item_key: UItemKey,
) -> Result<(), StatItemCheckError> {
    let u_item = ctx.u_data.items.get(item_key);
    let is_loaded = match u_item {
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

pub(super) fn check_item_key_autocharge_charge_drone_fighter_module(
    ctx: SvcCtx,
    item_key: UItemKey,
) -> Result<(), StatItemCheckError> {
    let u_item = ctx.u_data.items.get(item_key);
    let is_loaded = match u_item {
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
