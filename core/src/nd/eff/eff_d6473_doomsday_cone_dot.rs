use crate::{
    ac,
    ad::{AEffectBuffInfo, AEffectId},
    def::{AttrVal, Count, OF},
    ec,
    ed::EEffectId,
    misc::{DmgKinds, EffectSpec, Spool},
    nd::{
        NEffect, NEffectDmgKind, NEffectHc,
        eff::shared::{mods::make_dd_self_debuffs, opc::get_aoe_dd_neut_opc, proj_mult::get_dd_boson_proj_mult},
    },
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputComplex, OutputSimple},
    },
    ud::{UItem, UItemKey},
    util::floor_unerr,
};

const E_EFFECT_ID: EEffectId = ec::effects::DOOMSDAY_CONE_DOT;
const A_EFFECT_ID: AEffectId = ac::effects::DOOMSDAY_CONE_DOT;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_buff_info: Some(AEffectBuffInfo {
            custom: make_dd_self_debuffs().collect(),
            ..
        }),
        hc: NEffectHc {
            dmg_kind_getter: Some(internal_get_dmg_kind),
            normal_dmg_opc_getter: Some(internal_get_dmg_opc),
            neut_opc_getter: Some(get_aoe_dd_neut_opc),
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
    let mut dmg_em = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::EM_DMG)?;
    let mut dmg_therm = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::THERM_DMG)?;
    let mut dmg_kin = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::KIN_DMG)?;
    let mut dmg_expl = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::EXPL_DMG)?;
    let delay_s =
        calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::DOOMSDAY_WARNING_DURATION)? / OF(1000.0);
    if let Some(projectee_key) = projectee_key {
        // Projection reduction
        let proj_data = ctx.eff_projs.get_or_make_proj_data(
            ctx.u_data,
            EffectSpec::new(projector_key, projector_effect.get_key()),
            projectee_key,
        );
        let mult = get_dd_boson_proj_mult(ctx, calc, projector_key, projector_effect, projectee_key, proj_data);
        dmg_em *= mult;
        dmg_therm *= mult;
        dmg_kin *= mult;
        dmg_expl *= mult;
    }
    let interval_s =
        calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::DOOMSDAY_DMG_CYCLE_TIME)? / OF(1000.0);
    if interval_s > OF(0.0) {
        let duration_s =
            calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::DOOMSDAY_DMG_DURATION)? / OF(1000.0);
        let repeats = floor_unerr(duration_s / interval_s).into_inner() as Count;
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
                interval: interval_s,
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
