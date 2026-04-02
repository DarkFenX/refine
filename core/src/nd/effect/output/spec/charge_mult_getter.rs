use crate::{
    ad::{AItemGrpId, AItemId},
    num::{PValue, UnitInterval, Value},
    svc::{SvcCtx, calc::Calc},
    ud::UItemId,
};

#[derive(Copy, Clone)]
pub(crate) enum NEffectChargeMultGetter {
    AsbCap,
    AarRep,
}
impl NEffectChargeMultGetter {
    pub(crate) fn get(
        &self,
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        chargedness: UnitInterval,
    ) -> Option<PValue> {
        match self {
            Self::AsbCap => get_asb_cap(ctx, calc, item_uid),
            Self::AarRep => get_aar_rep(ctx, calc, item_uid, chargedness),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Getter implementations
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_asb_cap(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId) -> Option<PValue> {
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

fn get_aar_rep(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId, chargedness: UnitInterval) -> Option<PValue> {
    if let Some(charge_uid) = ctx.u_data.items.get(item_uid).get_charge_uid()
        && ctx.u_data.items.get(charge_uid).get_type_aid() == AItemId::NANITE_REPAIR_PASTE
        && let Some(rep_mult) = calc.get_item_oattr_oextra(ctx, item_uid, ctx.ac().charged_armor_dmg_mult)
    {
        return Some(PValue::from_value_clamped(rep_mult - Value::ONE) * chargedness.into_pvalue() + PValue::ONE);
    }
    None
}
