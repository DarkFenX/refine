use crate::{
    AttrVal, ac, ad,
    def::{ItemKey, OF},
    ec, ed,
    misc::{DmgKinds, Spool},
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeLoc, NEffectHc,
        eff::shared::proj_mult::{get_proj_attrs_normal, get_proj_mult_normal_unrestricted_s2s},
    },
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputSimple},
    },
};

const E_EFFECT_ID: ed::EEffectId = ec::effects::PROJECTILE_FIRED;
const A_EFFECT_ID: ad::AEffectId = ac::effects::PROJECTILE_FIRED;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        xt_get_proj_attrs: Some(get_proj_attrs_normal),
        hc: NEffectHc {
            charge: Some(NEffectCharge {
                location: NEffectChargeLoc::Loaded(NEffectChargeDepl::ChargeRate {
                    can_run_uncharged: false,
                }),
                activates_charge: false,
            }),
            get_proj_mult: Some(get_proj_mult_normal_unrestricted_s2s),
            get_normal_dmg_opc: Some(get_dmg_opc),
            ..
        },
        ..
    }
}

fn get_dmg_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: ItemKey,
    _projector_a_effect: &ad::AEffectRt,
    _spool: Option<Spool>,
    _projectee_key: Option<ItemKey>,
) -> Option<Output<DmgKinds<AttrVal>>> {
    let projector_uad_item = ctx.uad.items.get(projector_key);
    let charge_key = projector_uad_item.get_charge_key()?;
    let dmg_mult = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::DMG_MULT)?;
    let dmg_em = calc.get_item_attr_val_extra_opt(ctx, charge_key, &ac::attrs::EM_DMG)?;
    let dmg_therm = calc.get_item_attr_val_extra_opt(ctx, charge_key, &ac::attrs::THERM_DMG)?;
    let dmg_kin = calc.get_item_attr_val_extra_opt(ctx, charge_key, &ac::attrs::KIN_DMG)?;
    let dmg_expl = calc.get_item_attr_val_extra_opt(ctx, charge_key, &ac::attrs::EXPL_DMG)?;
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
