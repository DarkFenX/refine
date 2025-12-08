use crate::{
    ac,
    ad::{AEffectBuffInfo, AEffectId},
    def::{AttrVal, OF},
    ec,
    ed::EEffectId,
    misc::{DmgKinds, EffectSpec, Spool},
    nd::{
        NEffect, NEffectDmgKind, NEffectHc,
        effect::data::shared::{
            mods::make_dd_self_debuffs, opc::get_aoe_dd_side_neut_opc, proj_mult::get_aoe_dd_dmg_proj_mult,
        },
    },
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputSimple},
    },
    ud::{UItem, UItemKey},
};

const E_EFFECT_ID: EEffectId = ec::effects::DOOMSDAY_SLASH;
const A_EFFECT_ID: AEffectId = ac::effects::DOOMSDAY_SLASH;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_buff_info: Some(AEffectBuffInfo {
            full: make_dd_self_debuffs().collect(),
            ..
        }),
        hc: NEffectHc {
            dmg_kind_getter: Some(internal_get_dmg_kind),
            normal_dmg_opc_getter: Some(internal_get_dmg_opc),
            neut_opc_getter: Some(get_aoe_dd_side_neut_opc),
            ..
        },
        ..
    }
}

fn internal_get_dmg_kind(_u_item: &UItem) -> NEffectDmgKind {
    NEffectDmgKind::Superweapon
}

fn internal_get_dmg_opc(
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
    // Unlike other AoE doomsdays, reapers hit every ship only once, despite having damage ticks
    // spread over time. We also assume target is hit by first damage tick.
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
