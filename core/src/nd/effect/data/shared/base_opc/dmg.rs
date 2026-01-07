use crate::{
    misc::{Count, DmgKinds, PValue, Value},
    nd::{NEffectProjOpcSpec, effect::data::shared::proj_mult::get_aoe_dd_dmg_proj_mult},
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputComplex, OutputSimple},
    },
    ud::UItemId,
};

pub(in crate::nd::effect::data) fn get_instant_dmg_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    _effect: &REffect,
) -> Option<Output<DmgKinds<Value>>> {
    Some(Output::Simple(OutputSimple {
        amount: DmgKinds {
            em: calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().em_dmg, Value::ZERO)?,
            thermal: calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().therm_dmg, Value::ZERO)?,
            kinetic: calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().kin_dmg, Value::ZERO)?,
            explosive: calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().expl_dmg, Value::ZERO)?,
        },
        delay: PValue::ZERO,
    }))
}

pub(in crate::nd::effect::data) fn get_instant_charge_mult_dmg_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    _effect: &REffect,
) -> Option<Output<DmgKinds<Value>>> {
    let charge_uid = ctx.u_data.items.get(item_uid).get_charge_uid()?;
    let dmg_mult = calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().dmg_mult, Value::ONE)?;
    let dmg_em = calc.get_item_oattr_afb_oextra(ctx, charge_uid, ctx.ac().em_dmg, Value::ZERO)?;
    let dmg_therm = calc.get_item_oattr_afb_oextra(ctx, charge_uid, ctx.ac().therm_dmg, Value::ZERO)?;
    let dmg_kin = calc.get_item_oattr_afb_oextra(ctx, charge_uid, ctx.ac().kin_dmg, Value::ZERO)?;
    let dmg_expl = calc.get_item_oattr_afb_oextra(ctx, charge_uid, ctx.ac().expl_dmg, Value::ZERO)?;
    Some(Output::Simple(OutputSimple {
        amount: DmgKinds {
            em: dmg_em * dmg_mult,
            thermal: dmg_therm * dmg_mult,
            kinetic: dmg_kin * dmg_mult,
            explosive: dmg_expl * dmg_mult,
        },
        delay: PValue::ZERO,
    }))
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Doomsdays
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::nd::effect::data) fn get_direct_dd_dmg_opc_spec() -> NEffectProjOpcSpec<DmgKinds<Value>> {
    // Direct DDs have no range limitations
    NEffectProjOpcSpec {
        base: get_direct_dd_dmg_base_opc,
        ..
    }
}
fn get_direct_dd_dmg_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    _effect: &REffect,
) -> Option<Output<DmgKinds<Value>>> {
    Some(Output::Simple(OutputSimple {
        amount: DmgKinds {
            em: calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().em_dmg, Value::ZERO)?,
            thermal: calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().therm_dmg, Value::ZERO)?,
            kinetic: calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().kin_dmg, Value::ZERO)?,
            explosive: calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().expl_dmg, Value::ZERO)?,
        },
        delay: PValue::from_value_clamped(
            calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().dmg_delay_duration, Value::ZERO)? / Value::THOUSAND,
        ),
    }))
}

pub(in crate::nd::effect::data) fn get_aoe_dd_dmg_opc_spec() -> NEffectProjOpcSpec<DmgKinds<Value>> {
    // Direct DDs have no range limitations
    NEffectProjOpcSpec {
        base: get_aoe_dd_dmg_base_opc,
        proj_mult_str: Some(get_aoe_dd_dmg_proj_mult),
        ..
    }
}
fn get_aoe_dd_dmg_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    _effect: &REffect,
) -> Option<Output<DmgKinds<Value>>> {
    let dmg_em = calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().em_dmg, Value::ZERO)?;
    let dmg_therm = calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().therm_dmg, Value::ZERO)?;
    let dmg_kin = calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().kin_dmg, Value::ZERO)?;
    let dmg_expl = calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().expl_dmg, Value::ZERO)?;
    let delay_s = PValue::from_value_clamped(
        calc.get_item_oattr_afb_oextra(ctx, item_uid, ctx.ac().doomsday_warning_duration, Value::ZERO)?
            / Value::THOUSAND,
    );
    if let Some(interval_ms) = calc.get_item_oattr_oextra(ctx, item_uid, ctx.ac().doomsday_dmg_cycle_time)
        && interval_ms > Value::FLOAT_TOLERANCE
        && let Some(duration_ms) = calc.get_item_oattr_oextra(ctx, item_uid, ctx.ac().doomsday_dmg_duration)
    {
        let repeats = Count::from_value_trunced(duration_ms / interval_ms);
        if repeats > Count::ONE {
            return Some(Output::Complex(OutputComplex {
                amount: DmgKinds {
                    em: dmg_em,
                    thermal: dmg_therm,
                    kinetic: dmg_kin,
                    explosive: dmg_expl,
                },
                delay: delay_s,
                repeats,
                interval: PValue::from_value_clamped(interval_ms / Value::THOUSAND),
            }));
        }
    }
    Some(Output::Simple(OutputSimple {
        amount: DmgKinds {
            em: dmg_em,
            thermal: dmg_therm,
            kinetic: dmg_kin,
            explosive: dmg_expl,
        },
        delay: delay_s,
    }))
}
