use crate::{
    ac,
    def::{AttrVal, OF},
    misc::{DmgKinds, Spool},
    rd::REffect,
    svc::{
        SvcCtx,
        calc::Calc,
        output::{Output, OutputSimple},
    },
    ud::UItemKey,
};

pub(in crate::nd::eff) fn get_direct_dd_dmg_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    _projector_effect: &REffect,
    _spool: Option<Spool>,
    _projectee_key: Option<UItemKey>,
) -> Option<Output<DmgKinds<AttrVal>>> {
    // Direct DDs have no range limitations
    let dmg_em = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::EM_DMG)?;
    let dmg_therm = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::THERM_DMG)?;
    let dmg_kin = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::KIN_DMG)?;
    let dmg_expl = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::EXPL_DMG)?;
    let delay_s = calc.get_item_attr_val_extra_opt(ctx, projector_key, &ac::attrs::DMG_DELAY_DURATION)? / OF(1000.0);
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
