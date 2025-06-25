use crate::{
    ac,
    sol::{ItemKey, svc::SvcCtx, uad::item::UadItem},
};

pub(in crate::sol::svc::calc::modifier) fn revise_on_item_add_removal(
    ctx: &SvcCtx,
    affector_key: ItemKey,
    changed_item_key: ItemKey,
    changed_item: &UadItem,
) -> bool {
    match ctx.uad.items.get(affector_key) {
        UadItem::Module(module) => match module.get_charge_item_key() {
            Some(charge_key) => {
                changed_item_key == charge_key && changed_item.get_a_item_id() == ac::items::NANITE_REPAIR_PASTE
            }
            // No charge on AAR -> not changing anything
            None => false,
        },
        // The modifier isn't supposed to be carried on anything but a module
        _ => false,
    }
}
