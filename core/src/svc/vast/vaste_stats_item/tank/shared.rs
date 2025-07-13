use crate::{def::ItemKey, svc::SvcCtx, uad::UadItem};

pub struct StatTank<T> {
    pub shield: T,
    pub armor: T,
    pub hull: T,
}

pub(super) fn item_key_check(ctx: SvcCtx, item_key: ItemKey) -> Option<()> {
    let uad_item = ctx.uad.items.get(item_key);
    item_check(uad_item)
}

pub(super) fn item_check(uad_item: &UadItem) -> Option<()> {
    match uad_item {
        UadItem::Drone(_) | UadItem::Fighter(_) | UadItem::Ship(_) => Some(()),
        _ => None,
    }
}
