use crate::{
    def::OF,
    nd::NSpoolRaw,
    rd::RAttrKey,
    svc::{SvcCtx, calc::Calc},
    ud::UItemKey,
};

pub(in crate::nd::effect::data) fn get_rep_spool(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
) -> Option<NSpoolRaw> {
    get_spool(
        ctx,
        calc,
        item_key,
        ctx.ac().rep_mult_bonus_per_cycle,
        ctx.ac().rep_mult_bonus_max,
    )
}

pub(in crate::nd::effect::data) fn get_dmg_spool(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
) -> Option<NSpoolRaw> {
    get_spool(
        ctx,
        calc,
        item_key,
        ctx.ac().dmg_mult_bonus_per_cycle,
        ctx.ac().dmg_mult_bonus_max,
    )
}

fn get_spool(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    step_attr_key: Option<RAttrKey>,
    max_attr_key: Option<RAttrKey>,
) -> Option<NSpoolRaw> {
    let step = calc.get_item_attr_oextra(ctx, item_key, step_attr_key?)?;
    let max = calc.get_item_attr_oextra(ctx, item_key, max_attr_key?)?;
    match step > OF(0.0) && max > OF(0.0) {
        true => Some(NSpoolRaw { step, max }),
        false => None,
    }
}
