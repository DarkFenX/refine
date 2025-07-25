use crate::{
    def::{AttrVal, OF},
    misc::{DmgKinds, DpsProfile},
    svc::{
        SvcCtx,
        err::{KeyedItemKindVsStatError, KeyedItemLoadedError, StatItemCheckError},
    },
    ud::{UItem, UItemKey},
};

pub struct StatTank<T> {
    pub shield: T,
    pub armor: T,
    pub hull: T,
}

pub(super) fn item_key_check(ctx: SvcCtx, item_key: UItemKey) -> Result<(), StatItemCheckError> {
    let u_item = ctx.u_data.items.get(item_key);
    item_check(item_key, u_item)
}

pub(super) fn item_check(item_key: UItemKey, u_item: &UItem) -> Result<(), StatItemCheckError> {
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

pub(super) fn get_tanking_efficiency(resists: &DmgKinds<AttrVal>, incoming_dps: DpsProfile) -> Option<AttrVal> {
    let dealt = incoming_dps.get_sum_regular();
    let absorbed = incoming_dps.get_em() * resists.em
        + incoming_dps.get_thermal() * resists.thermal
        + incoming_dps.get_kinetic() * resists.kinetic
        + incoming_dps.get_explosive() * resists.explosive;
    let received = dealt - absorbed;
    match received > OF(0.0) {
        true => Some(dealt / received),
        false => None,
    }
}
