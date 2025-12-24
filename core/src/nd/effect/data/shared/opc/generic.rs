use crate::{
    def::{AttrVal, OF},
    rd::{RAttrKey, REffect},
    svc::{
        SvcCtx,
        calc::Calc,
        eff_funcs,
        output::{Output, OutputSimple},
    },
    ud::UItemKey,
};

pub(super) fn get_generic_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect: &REffect,
    attr_key: Option<RAttrKey>,
    applied_at_start: bool,
) -> Option<Output<AttrVal>> {
    let amount = calc.get_item_oattr_afb_odogma(ctx, item_key, attr_key, OF(0.0))?;
    let delay = match applied_at_start {
        true => OF(0.0),
        false => eff_funcs::get_effect_duration_s(ctx, calc, item_key, effect)?,
    };
    Some(Output::Simple(OutputSimple { amount, delay }))
}

pub(super) fn get_self_ilimit(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    attr_key: Option<RAttrKey>,
) -> Option<AttrVal> {
    let fit_key = ctx.u_data.items.get(item_key).get_fit_key()?;
    let ship_key = ctx.u_data.fits.get(fit_key).ship?;
    calc.get_item_oattr_oextra(ctx, ship_key, attr_key)
}

pub(super) fn get_proj_ilimit(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    attr_key: Option<RAttrKey>,
) -> Option<AttrVal> {
    calc.get_item_oattr_oextra(ctx, item_key, attr_key)
}
