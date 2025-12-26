use crate::{
    def::{AttrVal, OF},
    rd::{RAttrKey, REffect},
    svc::{
        SvcCtx,
        calc::Calc,
        funcs,
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
        false => funcs::get_effect_duration_s(ctx, calc, item_key, effect)?,
    };
    Some(Output::Simple(OutputSimple { amount, delay }))
}
