use crate::{
    def::{AttrVal, OF},
    misc::Ecm,
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputSimple},
    },
    ud::UItemKey,
};

pub(in crate::nd::effect::data) fn get_direct_ecm_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
) -> Option<Output<Ecm>> {
    let (radar, magnetometric, gravimetric, ladar) = get_ecm_values(ctx, calc, item_key)?;
    let duration = calc.get_item_oattr_afb_oextra(ctx, item_key, effect.duration_attr_key, OF(0.0))? / OF(1000.0);
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
    item_key: UItemKey,
    _effect: &REffect,
) -> Option<Output<Ecm>> {
    let (radar, magnetometric, gravimetric, ladar) = get_ecm_values(ctx, calc, item_key)?;
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
    item_key: UItemKey,
    _effect: &REffect,
) -> Option<Output<Ecm>> {
    let (radar, magnetometric, gravimetric, ladar) = get_ecm_values(ctx, calc, item_key)?;
    let duration = calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().doomsday_aoe_duration, OF(0.0))? / OF(1000.0);
    let delay =
        calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().doomsday_warning_duration, OF(0.0))? / OF(1000.0);
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
    item_key: UItemKey,
    _effect: &REffect,
) -> Option<Output<Ecm>> {
    let (radar, magnetometric, gravimetric, ladar) = get_ecm_values(ctx, calc, item_key)?;
    let duration = calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().ecm_jam_duration, OF(0.0))? / OF(1000.0);
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

fn get_ecm_values(ctx: SvcCtx, calc: &mut Calc, item_key: UItemKey) -> Option<(AttrVal, AttrVal, AttrVal, AttrVal)> {
    Some((
        calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().scan_radar_strength_bonus, OF(0.0))?,
        calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().scan_magnetometric_strength_bonus, OF(0.0))?,
        calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().scan_gravimetric_strength_bonus, OF(0.0))?,
        calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().scan_ladar_strength_bonus, OF(0.0))?,
    ))
}
