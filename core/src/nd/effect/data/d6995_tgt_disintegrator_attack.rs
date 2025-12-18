use crate::{
    ac,
    ad::AEffectId,
    def::{AttrVal, OF},
    ec,
    ed::EEffectId,
    misc::{DmgKinds, EffectSpec, Spool},
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeLoc, NEffectDmgKind,
        effect::{ResolvedSpool, data::shared::proj_mult::get_disintegrator_proj_mult},
    },
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputSimple},
    },
    ud::{UItem, UItemKey},
};

const E_EFFECT_ID: EEffectId = ec::effects::TGT_DISINTEGRATOR_ATTACK;
const A_EFFECT_ID: AEffectId = ac::effects::TGT_DISINTEGRATOR_ATTACK;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        charge: Some(NEffectCharge {
            location: NEffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate {
                can_run_uncharged: false,
            }),
            activates_charge: false,
        }),
        spool_resolver: Some(internal_get_resolved_spool),
        dmg_kind_getter: Some(internal_get_dmg_kind),
        normal_dmg_opc_getter: Some(get_dmg_opc),
        ..
    }
}

fn internal_get_dmg_kind(_u_item: &UItem) -> NEffectDmgKind {
    NEffectDmgKind::Turret
}

fn internal_get_resolved_spool(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    r_effect: &REffect,
    spool: Option<Spool>,
) -> Option<ResolvedSpool> {
    ResolvedSpool::try_build(
        ctx,
        calc,
        item_key,
        r_effect,
        spool,
        ctx.ac().dmg_mult_bonus_per_cycle,
        ctx.ac().dmg_mult_bonus_max,
    )
}

fn get_dmg_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    spool: Option<Spool>,
    projectee_key: Option<UItemKey>,
) -> Option<Output<DmgKinds<AttrVal>>> {
    let charge_key = ctx.u_data.items.get(projector_key).get_charge_key()?;
    let attr_consts = ctx.ac();
    let mut dmg_mult = calc.get_item_oattr_afb_oextra(ctx, projector_key, attr_consts.dmg_mult, OF(1.0))?;
    let dmg_em = calc.get_item_oattr_afb_oextra(ctx, charge_key, attr_consts.em_dmg, OF(0.0))?;
    let dmg_therm = calc.get_item_oattr_afb_oextra(ctx, charge_key, attr_consts.therm_dmg, OF(0.0))?;
    let dmg_kin = calc.get_item_oattr_afb_oextra(ctx, charge_key, attr_consts.kin_dmg, OF(0.0))?;
    let dmg_expl = calc.get_item_oattr_afb_oextra(ctx, charge_key, attr_consts.expl_dmg, OF(0.0))?;
    if let Some(resolved_spool) = internal_get_resolved_spool(ctx, calc, projector_key, projector_effect, spool) {
        dmg_mult *= resolved_spool.mult;
    }
    if let Some(projectee_key) = projectee_key {
        // Projection reduction
        let proj_data = ctx.eff_projs.get_or_make_proj_data(
            ctx.u_data,
            EffectSpec::new(projector_key, projector_effect.key),
            projectee_key,
        );
        dmg_mult *= get_disintegrator_proj_mult(ctx, calc, projector_key, projector_effect, projectee_key, proj_data);
    }
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
