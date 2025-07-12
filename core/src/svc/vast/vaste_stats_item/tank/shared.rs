use crate::{def::ItemKey, svc::SvcCtx, uad::UadItem};

pub struct StatTank<T> {
    pub shield: T,
    pub armor: T,
    pub hull: T,
}

pub(super) fn check_kind(ctx: SvcCtx, item_key: ItemKey) -> Option<()> {
    let uad_item = ctx.uad.items.get(item_key);
    match uad_item {
        UadItem::Drone(_) | UadItem::Fighter(_) | UadItem::Ship(_) => Some(()),
        _ => None,
    }
}
