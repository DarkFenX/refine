use crate::{
    ac,
    def::{AttrVal, OF},
    svc::{SvcCtx, calc::Calc, err::KeyedItemLoadedError},
    ud::{UItem, UItemKey, UNpcProp},
};

pub(crate) fn get_speed(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> Result<AttrVal, KeyedItemLoadedError> {
    let attr_id = match ctx.u_data.items.get(item_key) {
        UItem::Drone(u_drone) => match u_drone.get_prop_mode() {
            UNpcProp::Cruise => ac::attrs::ENTITY_CRUISE_SPEED,
            UNpcProp::Chase => ac::attrs::MAX_VELOCITY,
        },
        _ => ac::attrs::MAX_VELOCITY,
    };
    calc.get_item_attr_val_full(ctx, item_key, &attr_id)
        .map(|v| v.extra.max(OF(0.0)))
}

pub(crate) fn get_sig_radius(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
) -> Result<AttrVal, KeyedItemLoadedError> {
    let mut sig_radius = calc
        .get_item_attr_val_full(ctx, item_key, &ac::attrs::SIG_RADIUS)?
        .extra
        .max(OF(0.0));
    if let UItem::Drone(u_drone) = ctx.u_data.items.get(item_key)
        && matches!(u_drone.get_prop_mode(), UNpcProp::Chase)
    {
        sig_radius *= calc
            .get_item_attr_val_full(ctx, item_key, &ac::attrs::ENTITY_MAX_VELOCITY_SIG_RADIUS_MULT)
            .unwrap()
            .extra
            .max(OF(0.0))
    }
    Ok(sig_radius)
}
