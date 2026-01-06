use crate::{
    misc::{Ecm, PValue, Value},
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
    let duration = PValue::from_val_clamped(
        calc.get_item_oattr_afb_oextra(ctx, item_uid, effect.duration_attr_rid, Value::ZERO)? / Value::THOUSAND,
    );
    Some(Output::Simple(OutputSimple {
        amount: Ecm {
            radar,
            magnetometric,
            gravimetric,
            ladar,
            duration,
        },
        delay: PValue::ZERO,
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
            duration: PValue::ZERO,
        },
        delay: PValue::ZERO,
    }))
}

pub(in crate::nd::effect::data) fn get_aoe_ecm_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    _effect: &REffect,
) -> Option<Output<Ecm>> {
    let (radar, magnetometric, gravimetric, ladar) = get_ecm_values(ctx, calc, item_uid)?;
    let duration = PValue::from_val_clamped(
        calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().doomsday_aoe_duration, Value::ZERO)? / Value::THOUSAND,
    );
    let delay = PValue::from_val_clamped(
        calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().doomsday_warning_duration, Value::ZERO)?
            / Value::THOUSAND,
    );
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
    let duration = PValue::from_val_clamped(
        calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().ecm_jam_duration, Value::ZERO)? / Value::THOUSAND,
    );
    Some(Output::Simple(OutputSimple {
        amount: Ecm {
            radar,
            magnetometric,
            gravimetric,
            ladar,
            duration,
        },
        delay: PValue::ZERO,
    }))
}

fn get_ecm_values(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<(PValue, PValue, PValue, PValue)> {
    Some((
        PValue::from_val_clamped(calc.get_item_oattr_afb_oextra(
            ctx,
            item_uid,
            ctx.ac().scan_radar_strength_bonus,
            Value::ZERO,
        )?),
        PValue::from_val_clamped(calc.get_item_oattr_afb_oextra(
            ctx,
            item_uid,
            ctx.ac().scan_magnetometric_strength_bonus,
            Value::ZERO,
        )?),
        PValue::from_val_clamped(calc.get_item_oattr_afb_oextra(
            ctx,
            item_uid,
            ctx.ac().scan_gravimetric_strength_bonus,
            Value::ZERO,
        )?),
        PValue::from_val_clamped(calc.get_item_oattr_afb_oextra(
            ctx,
            item_uid,
            ctx.ac().scan_ladar_strength_bonus,
            Value::ZERO,
        )?),
    ))
}
