use crate::{
    ad::AItemId,
    num::{PValue, UnitInterval, Value},
    svc::{SvcCtx, calc::Calc},
    ud::UItemId,
};

pub(in crate::nd::effect::data) fn get_ancillary_armor_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    chargedness: UnitInterval,
) -> Option<PValue> {
    if let Some(charge_uid) = ctx.u_data.items.get(item_uid).get_charge_uid()
        && ctx.u_data.items.get(charge_uid).get_type_aid() == AItemId::NANITE_REPAIR_PASTE
        && let Some(rep_mult) = calc.get_item_oattr_oextra(ctx, item_uid, ctx.ac().charged_armor_dmg_mult)
    {
        return Some(PValue::from_value_clamped(rep_mult - Value::ONE) * chargedness.into_pvalue() + PValue::ONE);
    }
    None
}
