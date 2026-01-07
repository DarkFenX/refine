use crate::{
    misc::{PValue, Value},
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
) -> Option<Output<PValue>> {
    let amount = PValue::from_value_clamped(calc.get_item_oattr_afb_odogma(ctx, item_uid, attr_rid, Value::ZERO)?);
    let delay = match applied_at_start {
        true => PValue::ZERO,
        false => funcs::get_effect_duration_s(ctx, calc, item_uid, effect)?,
    };
    Some(Output::Simple(OutputSimple { amount, delay }))
}
