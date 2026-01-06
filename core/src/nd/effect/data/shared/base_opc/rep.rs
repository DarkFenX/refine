use super::generic::get_generic_base_opc;
use crate::{
    ac,
    def::{AttrVal, OF},
    rd::REffect,
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::UItemId,
};

pub(in crate::nd::effect::data) fn get_shield_rep_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
) -> Option<Output<AttrVal>> {
    get_generic_base_opc(ctx, calc, item_uid, effect, ctx.ac().shield_bonus, true)
}

pub(in crate::nd::effect::data) fn get_armor_rep_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
) -> Option<Output<AttrVal>> {
    get_generic_base_opc(ctx, calc, item_uid, effect, ctx.ac().armor_dmg_amount, false)
}

pub(in crate::nd::effect::data) fn get_hull_rep_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
) -> Option<Output<AttrVal>> {
    get_generic_base_opc(ctx, calc, item_uid, effect, ctx.ac().struct_dmg_amount, false)
}

pub(in crate::nd::effect::data) fn get_cap_trans_base_opc(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    effect: &REffect,
) -> Option<Output<AttrVal>> {
    get_generic_base_opc(ctx, calc, item_uid, effect, ctx.ac().power_transfer_amount, false)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Misc
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::nd::effect::data) fn get_ancillary_armor_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    chargedness: AttrVal,
) -> Option<AttrVal> {
    if let Some(charge_uid) = ctx.u_data.items.get(item_uid).get_charge_uid()
        && ctx.u_data.items.get(charge_uid).get_type_id() == AItemId::NANITE_REPAIR_PASTE
        && let Some(rep_mult) = calc.get_item_oattr_oextra(ctx, item_uid, ctx.ac().charged_armor_dmg_mult)
    {
        return Some((rep_mult - OF(1.0)) * chargedness + OF(1.0));
    }
    None
}
