use crate::{
    ad::AAttrId,
    def::OF,
    misc::{ResolvedSpool, Spool},
    rd::REffect,
    svc::{SvcCtx, calc::Calc, eff_funcs},
    ud::UItemKey,
};

pub(in crate::nd::effect) fn get_resolved_spool(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    r_effect: &REffect,
    spool: Option<Spool>,
    step_attr_id: &AAttrId,
    max_attr_id: &AAttrId,
) -> Option<ResolvedSpool> {
    let duration_s = eff_funcs::get_effect_duration_s(ctx, calc, item_key, r_effect)?;
    let spool = ctx.u_data.get_item_key_spool(item_key, spool);
    let spool_step = calc
        .get_item_attr_val_extra_opt(ctx, item_key, step_attr_id)
        .unwrap_or(OF(0.0));
    let spool_max = calc
        .get_item_attr_val_extra_opt(ctx, item_key, max_attr_id)
        .unwrap_or(OF(0.0));
    spool.resolve(spool_max, spool_step, duration_s)
}
