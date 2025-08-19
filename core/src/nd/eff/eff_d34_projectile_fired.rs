use crate::{
    ac,
    ad::AEffectId,
    def::{AttrVal, OF},
    ec,
    ed::EEffectId,
    misc::{DmgKinds, EffectSpec, Spool},
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeLoc, NEffectDmgKind, NEffectHc,
        eff::shared::proj_mult::get_proj_mult_normal_unrestricted_s2s,
    },
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputSimple},
    },
    ud::UItemKey,
};

const E_EFFECT_ID: EEffectId = ec::effects::PROJECTILE_FIRED;
const A_EFFECT_ID: AEffectId = ac::effects::PROJECTILE_FIRED;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        hc: NEffectHc {
            dmg_kind: Some(NEffectDmgKind::Turret),
            charge: Some(NEffectCharge {
                location: NEffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate {
                    can_run_uncharged: false,
                }),
                activates_charge: false,
            }),
            normal_dmg_opc_getter: Some(get_dmg_opc),
            ..
        },
        ..
    }
}

fn get_dmg_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_r_effect: &REffect,
    _spool: Option<Spool>,
    projectee_key: Option<UItemKey>,
) -> Option<Output<DmgKinds<AttrVal>>> {
    let projector_u_item = ctx.u_data.items.get(projector_key);
    let charge_key = projector_u_item.get_charge_key()?;
    let mut dmg_mult = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::DMG_MULT)?;
    let dmg_em = calc.get_item_attr_val_extra_opt(ctx, charge_key, &ac::attrs::EM_DMG)?;
    let dmg_therm = calc.get_item_attr_val_extra_opt(ctx, charge_key, &ac::attrs::THERM_DMG)?;
    let dmg_kin = calc.get_item_attr_val_extra_opt(ctx, charge_key, &ac::attrs::KIN_DMG)?;
    let dmg_expl = calc.get_item_attr_val_extra_opt(ctx, charge_key, &ac::attrs::EXPL_DMG)?;
    if let Some(projectee_key) = projectee_key {
        // Projection reduction
        let u_proj_data = ctx.eff_projs.get_or_make_proj_data(
            ctx.u_data,
            EffectSpec::new(projector_key, projector_r_effect.get_key()),
            projectee_key,
        );
        dmg_mult *= get_proj_mult_normal_unrestricted_s2s(ctx, calc, projector_key, projector_r_effect, u_proj_data);
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
