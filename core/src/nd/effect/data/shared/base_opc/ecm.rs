use crate::{
    def::{AttrVal, OF},
    misc::Ecm,
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputSimple},
    },
    ud::UItemId,
};

pub(in crate::nd::effect::data) fn get_direct_ecm_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
) -> Option<Output<Ecm>> {
    let (radar, magnetometric, gravimetric, ladar) = get_ecm_values(ctx, calc, item_uid)?;
    let duration = calc.get_item_oattr_afb_oextra(ctx, item_uid, effect.duration_attr_rid, OF(0.0))? / OF(1000.0);
    Some(Output::Simple(OutputSimple {
        amount: Ecm {
            radar,
            magnetometric,
            gravimetric,
            ladar,
            duration,
        },
        delay: OF(0.0),
    }))
}

pub(in crate::nd::effect::data) fn get_ecm_burst_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    _effect: &REffect,
) -> Option<Output<Ecm>> {
    let (radar, magnetometric, gravimetric, ladar) = get_ecm_values(ctx, calc, item_uid)?;
    Some(Output::Simple(OutputSimple {
        amount: Ecm {
            radar,
            magnetometric,
            gravimetric,
            ladar,
            duration: OF(0.0),
        },
        delay: OF(0.0),
    }))
}

pub(in crate::nd::effect::data) fn get_aoe_ecm_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    _effect: &REffect,
) -> Option<Output<Ecm>> {
    let (radar, magnetometric, gravimetric, ladar) = get_ecm_values(ctx, calc, item_uid)?;
    let duration = calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().doomsday_aoe_duration, OF(0.0))? / OF(1000.0);
    let delay =
        calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().doomsday_warning_duration, OF(0.0))? / OF(1000.0);
    Some(Output::Simple(OutputSimple {
        amount: Ecm {
            radar,
            magnetometric,
            gravimetric,
            ladar,
            duration,
        },
        delay,
    }))
}

pub(in crate::nd::effect::data) fn get_ecm_drone_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    _effect: &REffect,
) -> Option<Output<Ecm>> {
    let (radar, magnetometric, gravimetric, ladar) = get_ecm_values(ctx, calc, item_uid)?;
    let duration = calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().ecm_jam_duration, OF(0.0))? / OF(1000.0);
    Some(Output::Simple(OutputSimple {
        amount: Ecm {
            radar,
            magnetometric,
            gravimetric,
            ladar,
            duration,
        },
        delay: OF(0.0),
    }))
}

fn get_ecm_values(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<(AttrVal, AttrVal, AttrVal, AttrVal)> {
    Some((
        calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().scan_radar_strength_bonus, OF(0.0))?,
        calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().scan_magnetometric_strength_bonus, OF(0.0))?,
        calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().scan_gravimetric_strength_bonus, OF(0.0))?,
        calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().scan_ladar_strength_bonus, OF(0.0))?,
    ))
}
