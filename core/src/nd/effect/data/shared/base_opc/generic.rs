use crate::{
    def::{AttrVal, OF},
    rd::{RAttrId, REffect},
    svc::{
        SvcCtx,
        calc::Calc,
        funcs,
        output::{Output, OutputSimple},
    },
    ud::UItemId,
};

pub(super) fn get_generic_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
    attr_rid: Option<RAttrId>,
    applied_at_start: bool,
) -> Option<Output<AttrVal>> {
    let amount = calc.get_item_oattr_afb_odogma(ctx, item_uid, attr_rid, OF(0.0))?;
    let delay = match applied_at_start {
        true => OF(0.0),
        false => funcs::get_effect_duration_s(ctx, calc, item_uid, effect)?,
    };
    Some(Output::Simple(OutputSimple { amount, delay }))
}
