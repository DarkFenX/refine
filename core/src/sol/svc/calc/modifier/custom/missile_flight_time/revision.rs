use crate::sol::{ItemKey, svc::SvcCtx, uad::item::UadItem};

pub(in crate::sol::svc::calc::modifier) fn revise_on_item_add_removal(
    ctx: &SvcCtx,
    affector_key: ItemKey,
    changed_item: &UadItem,
) -> bool {
    match changed_item {
        UadItem::Ship(changed_ship) => {
            Some(changed_ship.get_fit_key()) == ctx.uad.items.get(affector_key).get_fit_key()
        }
        _ => false,
    }
}
