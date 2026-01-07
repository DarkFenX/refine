use super::traits::LimitAmount;
use crate::{
    misc::{PValue, UnitInterval, Value},
    rd::{RAttrId, REffect, REffectLocalOpcSpec},
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Data which stays the same through local effect cycles
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc) struct AggrLocalInvData<T>
where
    T: Copy,
{
    output: Output<T>,
    amount_limit: Option<Value>,
}
impl<T> AggrLocalInvData<T>
where
    T: Copy,
{
    pub(in crate::svc) fn try_make(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        effect: &REffect,
        ospec: &REffectLocalOpcSpec<T>,
    ) -> Option<Self> {
        Some(AggrLocalInvData {
            output: (ospec.base)(ctx, calc, item_uid, effect)?,
            amount_limit: get_ship_limit(ctx, calc, item_uid, ospec.limit_attr_rid),
        })
    }
}

fn get_ship_limit(ctx: SvcCtx, calc: &mut Calc, item_uid: UItemId, attr_rid: Option<RAttrId>) -> Option<Value> {
    let attr_rid = attr_rid?;
    let fit_uid = ctx.u_data.items.get(item_uid).get_fit_uid()?;
    let ship_uid = ctx.u_data.fits.get(fit_uid).ship?;
    calc.get_item_attr_oextra(ctx, ship_uid, attr_rid)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Helper functions
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc) fn get_local_output<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    ospec: &REffectLocalOpcSpec<T>,
    inv_local: &AggrLocalInvData<T>,
    chargeness: Option<UnitInterval>,
) -> Output<T>
where
    T: Copy + std::ops::MulAssign<PValue> + LimitAmount,
{
    let mut output = inv_local.output;
    // Chargedness
    if let Some(charge_mult_getter) = ospec.charge_mult
        && let Some(chargedness) = chargeness
        && let Some(charge_mult) = charge_mult_getter(ctx, calc, item_uid, chargedness)
    {
        output *= charge_mult;
    }
    // Limit
    if let Some(limit) = inv_local.amount_limit {
        output.limit_amount(limit);
    }
    output
}
