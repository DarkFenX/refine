use crate::{
    def::{AttrVal, Count, OF},
    misc::DmgKinds,
    nd::{NEffectProjOpcSpec, effect::data::shared::proj_mult::get_aoe_dd_dmg_proj_mult},
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputComplex, OutputSimple},
    },
    ud::UItemKey,
    util::{FLOAT_TOLERANCE, floor_unerr},
};

pub(in crate::nd::effect::data) fn get_instant_dmg_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    _effect: &REffect,
) -> Option<Output<DmgKinds<AttrVal>>> {
    Some(Output::Simple(OutputSimple {
        amount: DmgKinds {
            em: calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().em_dmg, OF(0.0))?,
            thermal: calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().therm_dmg, OF(0.0))?,
            kinetic: calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().kin_dmg, OF(0.0))?,
            explosive: calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().expl_dmg, OF(0.0))?,
        },
        delay: OF(0.0),
    }))
}

pub(in crate::nd::effect::data) fn get_instant_charge_mult_dmg_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    _effect: &REffect,
) -> Option<Output<DmgKinds<AttrVal>>> {
    let charge_key = ctx.u_data.items.get(item_key).get_charge_key()?;
    let dmg_mult = calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().dmg_mult, OF(1.0))?;
    let dmg_em = calc.get_item_oattr_afb_oextra(ctx, charge_key, ctx.ac().em_dmg, OF(0.0))?;
    let dmg_therm = calc.get_item_oattr_afb_oextra(ctx, charge_key, ctx.ac().therm_dmg, OF(0.0))?;
    let dmg_kin = calc.get_item_oattr_afb_oextra(ctx, charge_key, ctx.ac().kin_dmg, OF(0.0))?;
    let dmg_expl = calc.get_item_oattr_afb_oextra(ctx, charge_key, ctx.ac().expl_dmg, OF(0.0))?;
    Some(Output::Simple(OutputSimple {
        amount: DmgKinds {
            em: dmg_em * dmg_mult,
            thermal: dmg_therm * dmg_mult,
            kinetic: dmg_kin * dmg_mult,
            explosive: dmg_expl * dmg_mult,
        },
        delay: OF(0.0),
    }))
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Doomsdays
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::nd::effect::data) fn get_direct_dd_dmg_opc_spec() -> NEffectProjOpcSpec<DmgKinds<AttrVal>> {
    // Direct DDs have no range limitations
    NEffectProjOpcSpec {
        base: get_direct_dd_dmg_base_opc,
        ..
    }
}
fn get_direct_dd_dmg_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    _effect: &REffect,
) -> Option<Output<DmgKinds<AttrVal>>> {
    Some(Output::Simple(OutputSimple {
        amount: DmgKinds {
            em: calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().em_dmg, OF(0.0))?,
            thermal: calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().therm_dmg, OF(0.0))?,
            kinetic: calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().kin_dmg, OF(0.0))?,
            explosive: calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().expl_dmg, OF(0.0))?,
        },
        delay: calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().dmg_delay_duration, OF(0.0))? / OF(1000.0),
    }))
}

pub(in crate::nd::effect::data) fn get_aoe_dd_dmg_opc_spec() -> NEffectProjOpcSpec<DmgKinds<AttrVal>> {
    // Direct DDs have no range limitations
    NEffectProjOpcSpec {
        base: get_aoe_dd_dmg_base_opc,
        proj_mult_pre: Some(get_aoe_dd_dmg_proj_mult),
        ..
    }
}
fn get_aoe_dd_dmg_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    _effect: &REffect,
) -> Option<Output<DmgKinds<AttrVal>>> {
    let dmg_em = calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().em_dmg, OF(0.0))?;
    let dmg_therm = calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().therm_dmg, OF(0.0))?;
    let dmg_kin = calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().kin_dmg, OF(0.0))?;
    let dmg_expl = calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().expl_dmg, OF(0.0))?;
    let delay_s =
        calc.get_item_oattr_afb_oextra(ctx, item_key, ctx.ac().doomsday_warning_duration, OF(0.0))? / OF(1000.0);
    if let Some(interval_ms) = calc.get_item_oattr_oextra(ctx, item_key, ctx.ac().doomsday_dmg_cycle_time)
        && interval_ms > FLOAT_TOLERANCE
        && let Some(duration_ms) = calc.get_item_oattr_oextra(ctx, item_key, ctx.ac().doomsday_dmg_duration)
    {
        let repeats = floor_unerr(duration_ms / interval_ms).into_inner() as Count;
        if repeats >= 2 {
            return Some(Output::Complex(OutputComplex {
                amount: DmgKinds {
                    em: dmg_em,
                    thermal: dmg_therm,
                    kinetic: dmg_kin,
                    explosive: dmg_expl,
                },
                delay: delay_s,
                repeats,
                interval: interval_ms / OF(1000.0),
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
