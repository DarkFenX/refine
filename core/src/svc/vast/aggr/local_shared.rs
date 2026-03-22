use super::traits::LimitInstance;
use crate::{
    nd::NChargeMultGetter,
    num::{Count, PValue, UnitInterval, Value},
    rd::{RAttrId, REffect, REffectLocalOpcSpec},
    svc::{SvcCtx, calc::Calc, output::Output},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Data which stays the same through local effect cycles
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) struct AggrLocalInvData<T>
where
    T: Copy,
{
    output: Output<T>,
    instance_limit: Option<Value>,
}
impl<T> AggrLocalInvData<T>
where
    T: Copy,
{
    pub(super) fn try_make<BX>(
        ctx: SvcCtx,
        calc: &mut Calc,
        item_uid: UItemId,
        effect: &REffect,
        ospec: &REffectLocalOpcSpec<T, BX>,
        base_xargs: BX,
    ) -> Option<Self> {
        let output = (ospec.base)(ctx, calc, item_uid, effect, base_xargs)?;
        if output.get_instance_count() == Count::ZERO {
            return None;
        }
        Some(AggrLocalInvData {
            output,
            instance_limit: get_ship_limit(ctx, calc, item_uid, ospec.limit_attr_rid),
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
// Converter
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) struct LocalConverter<'u, 'p, 'c, 'o, 'i, T, BX>
where
    T: Copy,
{
    pub(super) ctx: SvcCtx<'u, 'p>,
    pub(super) calc: &'c mut Calc,
    pub(super) item_uid: UItemId,
    pub(super) ospec: &'o REffectLocalOpcSpec<T, BX>,
    pub(super) inv_local: &'i AggrLocalInvData<T>,
}
impl<'u, 'p, 'c, 'o, 'i, T, BX> LocalConverter<'u, 'p, 'c, 'o, 'i, T, BX>
where
    T: Copy,
{
    pub(super) fn new(
        ctx: SvcCtx<'u, 'p>,
        calc: &'c mut Calc,
        item_uid: UItemId,
        ospec: &'o REffectLocalOpcSpec<T, BX>,
        inv_local: &'i AggrLocalInvData<T>,
    ) -> Self {
        Self {
            ctx,
            calc,
            item_uid,
            ospec,
            inv_local,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Helper functions
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(super) fn get_local_output<T, BX>(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    ospec: &REffectLocalOpcSpec<T, BX>,
    inv_local: &AggrLocalInvData<T>,
    chargeness: Option<UnitInterval>,
) -> Output<T>
where
    T: Copy + std::ops::MulAssign<PValue> + LimitInstance,
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
    if let Some(limit) = inv_local.instance_limit {
        output.limit_instance(limit);
    }
    output
}
