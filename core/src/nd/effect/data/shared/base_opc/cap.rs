use crate::{
    ad::{AAttrId, AItemGrpId},
    nd::{NEffectProjMultGetter, NEffectProjOpcSpec, NEffectResist, NGeneralOutputGetter},
    num::{PValue, UnitInterval, Value},
    svc::{SvcCtx, calc::Calc},
    ud::UItemId,
};

pub(in crate::nd::effect::data) fn get_aoe_dd_side_neut_opc_spec() -> NEffectProjOpcSpec<NGeneralOutputGetter> {
    NEffectProjOpcSpec {
        base: NGeneralOutputGetter::NeutDdSideEffect,
        proj_mult_str: Some(NEffectProjMultGetter::AoeDdSideNeut),
        resist: Some(NEffectResist::Attr(AAttrId::DOOMSDAY_ENERGY_NEUT_RESIST_ID)),
        limit_attr_id: Some(AAttrId::CAPACITOR_CAPACITY),
        ..
    }
}

pub(in crate::nd::effect::data) fn get_ancillary_cap_mult(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    _chargedness: UnitInterval,
) -> Option<PValue> {
    if let Some(charge_uid) = ctx.u_data.items.get(item_uid).get_charge_uid()
        && ctx.u_data.items.get(charge_uid).get_group_id() == Some(AItemGrpId::CAPACITOR_BOOSTER_CHARGE)
        && let Some(cap_bonus_perc) = calc.get_item_oattr_oextra(ctx, charge_uid, ctx.ac().cap_need_bonus)
    {
        return Some(PValue::from_value_clamped(
            cap_bonus_perc.mul_add(Value::HUNDREDTH, Value::ONE),
        ));
    }
    None
}
