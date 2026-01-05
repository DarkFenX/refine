use crate::{
    def::{AttrVal, OF},
    svc::{SvcCtx, calc::Calc},
    ud::{UItem, UItemId, UNpcProp},
};

pub(crate) fn get_speed(ctx: SvcCtx, calc: &mut Calc, item_key: UItemId) -> AttrVal {
    let attr_key = match ctx.u_data.items.get(item_key) {
        UItem::Drone(u_drone) => match u_drone.get_npc_prop() {
            UNpcProp::Cruise => ctx.ac().entity_cruise_speed,
            UNpcProp::Chase => ctx.ac().max_velocity,
        },
        _ => ctx.ac().max_velocity,
    };
    calc.get_item_oattr_ffb_extra(ctx, item_key, attr_key, OF(0.0))
        .max(OF(0.0))
}

pub(crate) fn get_sig_radius(ctx: SvcCtx, calc: &mut Calc, item_key: UItemId) -> AttrVal {
    let mut sig_radius = calc
        .get_item_oattr_ffb_extra(ctx, item_key, ctx.ac().sig_radius, OF(0.0))
        .max(OF(0.0));
    if let UItem::Drone(u_drone) = ctx.u_data.items.get(item_key)
        && matches!(u_drone.get_npc_prop(), UNpcProp::Chase)
    {
        sig_radius *= calc
            .get_item_oattr_ffb_extra(ctx, item_key, ctx.ac().entity_max_velocity_sig_radius_mult, OF(1.0))
            .max(OF(0.0))
    }
    sig_radius
}
