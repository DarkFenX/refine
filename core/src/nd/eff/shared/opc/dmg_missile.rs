use crate::{
    ac,
    def::{AttrVal, OF},
    misc::{DmgKinds, EffectSpec},
    nd::NProjMultGetter,
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputSimple},
    },
    ud::UItemKey,
};

pub(in crate::nd::eff) fn get_missile_dmg_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    projectee_key: Option<UItemKey>,
    proj_mult_getter: NProjMultGetter,
) -> Option<Output<DmgKinds<AttrVal>>> {
    let mut dmg_em = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::EM_DMG)?;
    let mut dmg_therm = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::THERM_DMG)?;
    let mut dmg_kin = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::KIN_DMG)?;
    let mut dmg_expl = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::EXPL_DMG)?;
    if let Some(projectee_key) = projectee_key {
        // Projection reduction
        let proj_data = ctx.eff_projs.get_or_make_proj_data(
            ctx.u_data,
            EffectSpec::new(projector_key, projector_effect.get_key()),
            projectee_key,
        );
        let mult = proj_mult_getter(ctx, calc, projector_key, projector_effect, projectee_key, proj_data);
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
