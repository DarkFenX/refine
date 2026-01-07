use crate::{
    misc::{NpcProp, PValue, Value},
    svc::{SvcCtx, calc::Calc},
    ud::UItemId,
};

pub(crate) fn get_speed(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> PValue {
    let attr_rid = match ctx.u_data.get_item_npc_prop(item_uid) {
        Some(NpcProp::Cruise) => ctx.ac().entity_cruise_speed,
        _ => ctx.ac().max_velocity,
    };
    PValue::from_value_clamped(calc.get_item_oattr_ffb_extra(ctx, item_uid, attr_rid, Value::ZERO))
}

pub(crate) fn get_sig_radius(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> PValue {
    let mut sig_radius =
        PValue::from_value_clamped(calc.get_item_oattr_ffb_extra(ctx, item_uid, ctx.ac().sig_radius, Value::ZERO));
    if let Some(NpcProp::Cruise) = ctx.u_data.get_item_npc_prop(item_uid) {
        sig_radius *= PValue::from_value_clamped(calc.get_item_oattr_ffb_extra(
            ctx,
            item_uid,
            ctx.ac().entity_max_velocity_sig_radius_mult,
            Value::ONE,
        ))
    }
    sig_radius
}
