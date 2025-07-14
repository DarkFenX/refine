use crate::{
    def::{AttrVal, ItemKey, OF},
    misc::{DmgKinds, DpsProfile},
    svc::{
        SvcCtx,
        err::{KeyedItemKindVsStatError, KeyedItemLoadedError, StatItemCheckError},
    },
    uad::UadItem,
};

pub struct StatTank<T> {
    pub shield: T,
    pub armor: T,
    pub hull: T,
}

pub(super) fn item_key_check(ctx: SvcCtx, item_key: ItemKey) -> Result<(), StatItemCheckError> {
    let uad_item = ctx.uad.items.get(item_key);
    item_check(item_key, uad_item)
}

pub(super) fn item_check(item_key: ItemKey, uad_item: &UadItem) -> Result<(), StatItemCheckError> {
    let is_loaded = match uad_item {
        UadItem::Drone(uad_drone) => uad_drone.is_loaded(),
        UadItem::Fighter(uad_fighter) => uad_fighter.is_loaded(),
        UadItem::Ship(uad_ship) => uad_ship.is_loaded(),
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
