use crate::{
    ac,
    ad::AEffectId,
    def::{AttrVal, OF},
    ec,
    ed::EEffectId,
    misc::{DmgKinds, Spool},
    nd::{
        NEffect, NEffectDmgKind, NEffectHc,
        eff::shared::proj_mult::{get_proj_attrs_simple, get_proj_mult_simple_s2s},
    },
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputSimple},
    },
    ud::UItemKey,
};

const E_EFFECT_ID: EEffectId = ec::effects::EMP_WAVE;
const A_EFFECT_ID: AEffectId = ac::effects::EMP_WAVE;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        xt_get_proj_attrs: Some(get_proj_attrs_simple),
        hc: NEffectHc {
            dmg_kind: Some(NEffectDmgKind::Smartbomb),
            proj_mult_getter: Some(get_proj_mult_simple_s2s),
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
    _projector_r_effect: &REffect,
    _spool: Option<Spool>,
    _projectee_key: Option<UItemKey>,
) -> Option<Output<DmgKinds<AttrVal>>> {
    let dmg_em = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::EM_DMG)?;
    let dmg_therm = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::THERM_DMG)?;
    let dmg_kin = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::KIN_DMG)?;
    let dmg_expl = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::EXPL_DMG)?;
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
