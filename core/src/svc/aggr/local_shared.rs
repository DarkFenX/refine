use super::traits::LimitAmount;
use crate::{
    def::AttrVal,
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
    amount_limit: Option<AttrVal>,
}
impl<T> AggrLocalInvData<T>
where
    T: Copy,
{
    pub(in crate::svc) fn try_make(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_key: UItemId,
        effect: &REffect,
        ospec: &REffectLocalOpcSpec<T>,
    ) -> Option<Self> {
        Some(AggrLocalInvData {
            output: (ospec.base)(ctx, calc, item_key, effect)?,
            amount_limit: get_ship_limit(ctx, calc, item_key, ospec.limit_attr_rid),
        })
    }
}

fn get_ship_limit(ctx: SvcCtx, calc: &mut Calc, item_key: UItemId, attr_key: Option<RAttrId>) -> Option<AttrVal> {
    let attr_key = attr_key?;
    let fit_key = ctx.u_data.items.get(item_key).get_fit_key()?;
    let ship_key = ctx.u_data.fits.get(fit_key).ship?;
    calc.get_item_attr_oextra(ctx, ship_key, attr_key)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Helper functions
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::svc) fn get_local_output<T>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemId,
    ospec: &REffectLocalOpcSpec<T>,
    inv_local: &AggrLocalInvData<T>,
    chargeness: Option<AttrVal>,
) -> Output<T>
where
    T: Copy + std::ops::MulAssign<AttrVal> + LimitAmount,
{
    let mut output = inv_local.output;
    // Chargedness
    if let Some(charge_mult_getter) = ospec.charge_mult
        && let Some(chargedness) = chargeness
        && let Some(charge_mult) = charge_mult_getter(ctx, calc, item_key, chargedness)
    {
        output *= charge_mult;
    }
    // Limit
    if let Some(limit) = inv_local.amount_limit {
        output.limit_amount(limit);
    }
    output
}
