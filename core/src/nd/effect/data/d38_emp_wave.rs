use crate::{
    ac,
    ad::AEffectId,
    def::{AttrVal, OF},
    ec,
    ed::EEffectId,
    misc::{DmgKinds, EffectSpec, Spool},
    nd::{NEffect, NEffectDmgKind, NEffectHc, effect::data::shared::proj_mult::get_simple_s2s_noapp_proj_mult},
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputSimple},
    },
    ud::{UItem, UItemKey},
};

const E_EFFECT_ID: EEffectId = ec::effects::EMP_WAVE;
const A_EFFECT_ID: AEffectId = ac::effects::EMP_WAVE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        hc: NEffectHc {
            dmg_kind_getter: Some(internal_get_dmg_kind),
            normal_dmg_opc_getter: Some(get_dmg_opc),
            ..
        },
        ..
    }
}

fn internal_get_dmg_kind(_u_item: &UItem) -> NEffectDmgKind {
    NEffectDmgKind::Smartbomb
}

fn get_dmg_opc(
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
    if let Some(projectee_key) = projectee_key {
        // Projection/application reduction
        let proj_data = ctx.eff_projs.get_or_make_proj_data(
            ctx.u_data,
            EffectSpec::new(projector_key, projector_effect.key),
            projectee_key,
        );
        let mult = get_simple_s2s_noapp_proj_mult(ctx, calc, projector_key, projector_effect, projectee_key, proj_data);
        dmg_em *= mult;
        dmg_therm *= mult;
        dmg_kin *= mult;
        dmg_expl *= mult;
    }
    Some(Output::Simple(OutputSimple {
        amount: DmgKinds {
            em: dmg_em,
            thermal: dmg_therm,
            kinetic: dmg_kin,
            explosive: dmg_expl,
        },
        delay: OF(0.0),
    }))
}
