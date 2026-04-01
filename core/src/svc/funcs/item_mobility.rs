use crate::{
    misc::NpcProp,
    num::{PValue, Value},
    svc::{SvcCtx, calc::Calc},
    ud::UItemId,
};

pub(crate) fn get_speed(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> PValue {
    let attr_rid = match get_npc_prop(ctx, item_uid) {
        Some(NpcProp::Cruise) => ctx.ac().entity_cruise_speed,
        _ => ctx.ac().max_velocity,
    };
    PValue::from_value_clamped(calc.get_item_oattr_ffb_extra(ctx, item_uid, attr_rid, Value::ZERO))
}

pub(crate) fn get_sig_radius(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> PValue {
    let mut sig_radius =
        PValue::from_value_clamped(calc.get_item_oattr_ffb_extra(ctx, item_uid, ctx.ac().sig_radius, Value::ZERO));
    if let Some(NpcProp::Chase) = get_npc_prop(ctx, item_uid) {
        sig_radius *= PValue::from_value_clamped(calc.get_item_oattr_ffb_extra(
            ctx,
            item_uid,
            ctx.ac().entity_max_velocity_sig_radius_mult,
            Value::ONE,
        ))
    }
    sig_radius
}

// Get the setting only for items which can use it
fn get_npc_prop(ctx: SvcCtx, item_uid: UItemId) -> Option<NpcProp> {
    let u_item = ctx.u_data.items.get(item_uid);
    if let Some(item_axt) = u_item.get_axt()
        && !item_axt.entity_mwd
    {
        return None;
    }
    ctx.u_data.get_item_npc_prop(u_item)
}
