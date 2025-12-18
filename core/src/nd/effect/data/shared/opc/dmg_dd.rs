use crate::{
    def::{AttrVal, Count, OF},
    misc::{DmgKinds, EffectSpec, Spool},
    nd::effect::data::shared::proj_mult::get_aoe_dd_dmg_proj_mult,
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputComplex, OutputSimple},
    },
    ud::UItemKey,
    util::{FLOAT_TOLERANCE, floor_unerr},
};

pub(in crate::nd::effect::data) fn get_direct_dd_dmg_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    _projector_effect: &REffect,
    _spool: Option<Spool>,
    _projectee_key: Option<UItemKey>,
) -> Option<Output<DmgKinds<AttrVal>>> {
    // Direct DDs have no range limitations
    let attr_consts = ctx.ac();
    let dmg_em = calc.get_item_oattr_afb_oextra(ctx, projector_key, attr_consts.em_dmg, OF(0.0))?;
    let dmg_therm = calc.get_item_oattr_afb_oextra(ctx, projector_key, attr_consts.therm_dmg, OF(0.0))?;
    let dmg_kin = calc.get_item_oattr_afb_oextra(ctx, projector_key, attr_consts.kin_dmg, OF(0.0))?;
    let dmg_expl = calc.get_item_oattr_afb_oextra(ctx, projector_key, attr_consts.expl_dmg, OF(0.0))?;
    let delay_s =
        calc.get_item_oattr_afb_oextra(ctx, projector_key, attr_consts.dmg_delay_duration, OF(0.0))? / OF(1000.0);
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

pub(in crate::nd::effect::data) fn get_aoe_dd_dmg_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    _spool: Option<Spool>,
    projectee_key: Option<UItemKey>,
) -> Option<Output<DmgKinds<AttrVal>>> {
    let attr_consts = ctx.ac();
    let mut dmg_em = calc.get_item_oattr_afb_oextra(ctx, projector_key, attr_consts.em_dmg, OF(0.0))?;
    let mut dmg_therm = calc.get_item_oattr_afb_oextra(ctx, projector_key, attr_consts.therm_dmg, OF(0.0))?;
    let mut dmg_kin = calc.get_item_oattr_afb_oextra(ctx, projector_key, attr_consts.kin_dmg, OF(0.0))?;
    let mut dmg_expl = calc.get_item_oattr_afb_oextra(ctx, projector_key, attr_consts.expl_dmg, OF(0.0))?;
    let delay_s = calc.get_item_oattr_afb_oextra(ctx, projector_key, attr_consts.doomsday_warning_duration, OF(0.0))?
        / OF(1000.0);
    if let Some(projectee_key) = projectee_key {
        // Projection reduction
        let proj_data = ctx.eff_projs.get_or_make_proj_data(
            ctx.u_data,
            EffectSpec::new(projector_key, projector_effect.key),
            projectee_key,
        );
        let mult = get_aoe_dd_dmg_proj_mult(ctx, calc, projector_key, projector_effect, projectee_key, proj_data);
        dmg_em *= mult;
        dmg_therm *= mult;
        dmg_kin *= mult;
        dmg_expl *= mult;
    }
    if let Some(interval_ms) = calc.get_item_oattr_oextra(ctx, projector_key, attr_consts.doomsday_dmg_cycle_time)
        && interval_ms > FLOAT_TOLERANCE
        && let Some(duration_ms) = calc.get_item_oattr_oextra(ctx, projector_key, attr_consts.doomsday_dmg_duration)
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
