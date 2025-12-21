use crate::{
    def::{AttrVal, OF},
    svc::{SvcCtx, calc::Calc, vast::StatDmgBreacher},
    ud::UItemKey,
};

pub(in crate::svc::vast) fn apply_breacher(
    ctx: SvcCtx,
    calc: &mut Calc,
    breacher_raw: StatDmgBreacher,
    projectee_key: UItemKey,
) -> AttrVal {
    let attr_consts = ctx.ac();
    let hp_shield = calc.get_item_oattr_ffb_extra(ctx, projectee_key, attr_consts.shield_capacity, OF(0.0));
    let hp_armor = calc.get_item_oattr_ffb_extra(ctx, projectee_key, attr_consts.armor_hp, OF(0.0));
    let hp_hull = calc.get_item_oattr_ffb_extra(ctx, projectee_key, attr_consts.hp, OF(0.0));
    breacher_raw
        .absolute_max
        .min(breacher_raw.relative_max * (hp_shield + hp_armor + hp_hull))
}
